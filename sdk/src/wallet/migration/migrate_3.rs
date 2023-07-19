// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::*;

pub struct Migrate;

#[async_trait]
impl MigrationData for Migrate {
    const ID: usize = 3;
    const SDK_VERSION: &'static str = "1.0.0-rc.0";
    const DATE: time::Date = time::macros::date!(2023 - 07 - 18);
}

#[async_trait]
#[cfg(feature = "storage")]
impl Migration<crate::wallet::storage::Storage> for Migrate {
    async fn migrate(storage: &crate::wallet::storage::Storage) -> Result<()> {
        use crate::wallet::storage::constants::{
            ACCOUNTS_INDEXATION_KEY, ACCOUNT_INDEXATION_KEY, WALLET_INDEXATION_KEY,
        };

        if let Some(account_indexes) = storage.get::<Vec<u32>>(ACCOUNTS_INDEXATION_KEY).await? {
            for account_index in account_indexes {
                if let Some(mut account) = storage
                    .get::<serde_json::Value>(&format!("{ACCOUNT_INDEXATION_KEY}{account_index}"))
                    .await?
                {
                    migrate_account(&mut account)?;

                    storage
                        .set(&format!("{ACCOUNT_INDEXATION_KEY}{account_index}"), &account)
                        .await?;
                }
            }
        }

        if let Some(mut wallet) = storage.get::<serde_json::Value>(WALLET_INDEXATION_KEY).await? {
            let params = wallet["client_options"]["protocolParameters"].as_object_mut().unwrap();
            if let Some(version) = params.remove("protocol_version") {
                params.insert("version".to_owned(), version);
            }
            ConvertNetworkName::check(&mut wallet["client_options"]["protocolParameters"]["network_name"])?;
            ConvertTokenSupply::check(&mut wallet["client_options"]["protocolParameters"]["token_supply"])?;
            rename_keys(&mut wallet);

            storage.set(WALLET_INDEXATION_KEY, &wallet).await?;
        }
        Ok(())
    }
}

#[async_trait]
#[cfg(feature = "stronghold")]
impl Migration<crate::client::stronghold::StrongholdAdapter> for Migrate {
    async fn migrate(storage: &crate::client::stronghold::StrongholdAdapter) -> Result<()> {
        use crate::{
            client::storage::StorageAdapter,
            wallet::core::operations::stronghold_backup::stronghold_snapshot::{ACCOUNTS_KEY, CLIENT_OPTIONS_KEY},
        };

        if let Some(mut accounts) = storage.get::<Vec<serde_json::Value>>(ACCOUNTS_KEY).await? {
            for account in &mut accounts {
                migrate_account(account)?;
            }
            storage.set(ACCOUNTS_KEY, &accounts).await?;
        }

        if let Some(mut client_options) = storage.get::<serde_json::Value>(CLIENT_OPTIONS_KEY).await? {
            let params = client_options["protocolParameters"].as_object_mut().unwrap();
            if let Some(version) = params.remove("protocol_version") {
                params.insert("version".to_owned(), version);
            }
            ConvertNetworkName::check(&mut client_options["protocolParameters"]["network_name"])?;
            ConvertTokenSupply::check(&mut client_options["protocolParameters"]["token_supply"])?;
            rename_keys(&mut client_options);

            storage.set(CLIENT_OPTIONS_KEY, &client_options).await?;
        }
        Ok(())
    }
}

fn migrate_account(account: &mut serde_json::Value) -> Result<()> {
    for output_data in account["outputs"]
        .as_object_mut()
        .ok_or(Error::Storage("malformatted outputs".to_owned()))?
        .values_mut()
    {
        ConvertOutputData::check(output_data)?;
    }

    for output_data in account["unspentOutputs"]
        .as_object_mut()
        .ok_or(Error::Storage("malformatted unspent outputs".to_owned()))?
        .values_mut()
    {
        ConvertOutputData::check(output_data)?;
    }

    for transaction in account["transactions"]
        .as_object_mut()
        .ok_or(Error::Storage("malformatted transactions".to_owned()))?
        .values_mut()
    {
        ConvertTransaction::check(transaction)?;
    }

    for transaction in account["incomingTransactions"]
        .as_object_mut()
        .ok_or(Error::Storage("malformatted incoming transactions".to_owned()))?
        .values_mut()
    {
        ConvertTransaction::check(transaction)?;
    }

    if let Some(foundries) = account.get_mut("nativeTokenFoundries") {
        for foundry_output in foundries
            .as_object_mut()
            .ok_or(Error::Storage("malformatted foundry outputs".to_owned()))?
            .values_mut()
        {
            ConvertFoundryOutput::check(foundry_output)?;
        }
    }

    Ok(())
}

mod types {
    use derive_more::From;
    use serde::{Deserialize, Serialize};
    use serde_json::Value;

    use super::{
        migrate_0::types::{OutputId, OutputMetadata, OutputMetadataDto},
        migrate_1::types::NewNativeToken as NativeToken,
        migrate_2::types::Bip44,
    };

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct OutputData {
        pub output_id: Value,
        pub metadata: OutputMetadata,
        pub output: Output,
        pub is_spent: bool,
        pub address: Address,
        pub network_id: u64,
        pub remainder: bool,
        pub chain: Option<Bip44>,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct OutputDataDto {
        pub output_id: Value,
        pub metadata: OutputMetadataDto,
        pub output: OutputDto,
        pub is_spent: bool,
        pub address: AddressDto,
        pub network_id: String,
        pub remainder: bool,
        pub chain: Option<Bip44>,
    }

    #[derive(Deserialize)]
    #[serde(tag = "type", content = "data")]
    pub enum Output {
        Treasury(TreasuryOutput),
        Basic(BasicOutput),
        Alias(AliasOutput),
        Foundry(FoundryOutput),
        Nft(NftOutput),
    }

    #[derive(Serialize, From)]
    #[serde(untagged)]
    pub enum OutputDto {
        Treasury(TreasuryOutputDto),
        Basic(BasicOutputDto),
        Alias(AliasOutputDto),
        Foundry(FoundryOutputDto),
        Nft(NftOutputDto),
    }

    impl From<Output> for OutputDto {
        fn from(value: Output) -> Self {
            match value {
                Output::Treasury(o) => TreasuryOutputDto::from(o).into(),
                Output::Basic(o) => BasicOutputDto::from(o).into(),
                Output::Alias(o) => AliasOutputDto::from(o).into(),
                Output::Foundry(o) => FoundryOutputDto::from(o).into(),
                Output::Nft(o) => NftOutputDto::from(o).into(),
            }
        }
    }

    impl<'de> Deserialize<'de> for OutputDto {
        fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            let value = Value::deserialize(d)?;
            Ok(
                match value
                    .get("type")
                    .and_then(Value::as_u64)
                    .ok_or_else(|| serde::de::Error::custom("invalid output type"))? as u8
                {
                    2 => {
                        Self::Treasury(TreasuryOutputDto::deserialize(value).map_err(|e| {
                            serde::de::Error::custom(format!("cannot deserialize treasury output: {e}"))
                        })?)
                    }
                    3 => Self::Basic(
                        BasicOutputDto::deserialize(value)
                            .map_err(|e| serde::de::Error::custom(format!("cannot deserialize basic output: {e}")))?,
                    ),
                    4 => Self::Alias(
                        AliasOutputDto::deserialize(value)
                            .map_err(|e| serde::de::Error::custom(format!("cannot deserialize alias output: {e}")))?,
                    ),
                    5 => Self::Foundry(
                        FoundryOutputDto::deserialize(value)
                            .map_err(|e| serde::de::Error::custom(format!("cannot deserialize foundry output: {e}")))?,
                    ),
                    6 => Self::Nft(
                        NftOutputDto::deserialize(value)
                            .map_err(|e| serde::de::Error::custom(format!("cannot deserialize NFT output: {e}")))?,
                    ),
                    _ => return Err(serde::de::Error::custom("invalid output type")),
                },
            )
        }
    }

    #[derive(Deserialize)]
    pub struct TreasuryOutput {
        pub amount: u64,
    }

    #[derive(Serialize, Deserialize)]
    pub struct TreasuryOutputDto {
        #[serde(rename = "type")]
        pub kind: u8,
        pub amount: String,
    }

    impl From<TreasuryOutput> for TreasuryOutputDto {
        fn from(value: TreasuryOutput) -> Self {
            Self {
                kind: 2,
                amount: value.amount.to_string(),
            }
        }
    }

    #[derive(Deserialize)]
    pub struct BasicOutput {
        pub amount: u64,
        pub native_tokens: BoxedSlicePrefix<NativeToken>,
        pub unlock_conditions: BoxedSlicePrefix<UnlockCondition>,
        pub features: BoxedSlicePrefix<Feature>,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct BasicOutputDto {
        #[serde(rename = "type")]
        pub kind: u8,
        pub amount: String,
        #[serde(skip_serializing_if = "Vec::is_empty", default)]
        pub native_tokens: Vec<NativeToken>,
        pub unlock_conditions: Vec<UnlockConditionDto>,
        #[serde(skip_serializing_if = "Vec::is_empty", default)]
        pub features: Vec<FeatureDto>,
    }

    impl From<BasicOutput> for BasicOutputDto {
        fn from(value: BasicOutput) -> Self {
            Self {
                kind: 3,
                amount: value.amount.to_string(),
                native_tokens: value.native_tokens.inner,
                unlock_conditions: value.unlock_conditions.inner.into_iter().map(Into::into).collect(),
                features: value.features.inner.into_iter().map(Into::into).collect(),
            }
        }
    }

    #[derive(Deserialize)]
    pub struct AliasOutput {
        pub amount: u64,
        pub native_tokens: BoxedSlicePrefix<NativeToken>,
        pub alias_id: Value,
        pub state_index: u32,
        pub state_metadata: BoxedSlicePrefix<u8>,
        pub foundry_counter: u32,
        pub unlock_conditions: BoxedSlicePrefix<UnlockCondition>,
        pub features: BoxedSlicePrefix<Feature>,
        pub immutable_features: BoxedSlicePrefix<Feature>,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct AliasOutputDto {
        #[serde(rename = "type")]
        pub kind: u8,
        pub amount: String,
        #[serde(skip_serializing_if = "Vec::is_empty", default)]
        pub native_tokens: Vec<NativeToken>,
        pub alias_id: Value,
        pub state_index: u32,
        #[serde(skip_serializing_if = "String::is_empty", default)]
        pub state_metadata: String,
        pub foundry_counter: u32,
        pub unlock_conditions: Vec<UnlockConditionDto>,
        #[serde(skip_serializing_if = "Vec::is_empty", default)]
        pub features: Vec<FeatureDto>,
        #[serde(skip_serializing_if = "Vec::is_empty", default)]
        pub immutable_features: Vec<FeatureDto>,
    }

    impl From<AliasOutput> for AliasOutputDto {
        fn from(value: AliasOutput) -> Self {
            Self {
                kind: 4,
                amount: value.amount.to_string(),
                native_tokens: value.native_tokens.inner,
                alias_id: value.alias_id,
                state_index: value.state_index,
                state_metadata: prefix_hex::encode(value.state_metadata.inner),
                foundry_counter: value.foundry_counter,
                unlock_conditions: value.unlock_conditions.inner.into_iter().map(Into::into).collect(),
                features: value.features.inner.into_iter().map(Into::into).collect(),
                immutable_features: value.immutable_features.inner.into_iter().map(Into::into).collect(),
            }
        }
    }

    #[derive(Deserialize)]
    pub struct FoundryOutput {
        pub amount: u64,
        pub native_tokens: BoxedSlicePrefix<NativeToken>,
        pub serial_number: u32,
        pub token_scheme: TokenScheme,
        pub unlock_conditions: BoxedSlicePrefix<UnlockCondition>,
        pub features: BoxedSlicePrefix<Feature>,
        pub immutable_features: BoxedSlicePrefix<Feature>,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct FoundryOutputDto {
        #[serde(rename = "type")]
        pub kind: u8,
        pub amount: String,
        #[serde(skip_serializing_if = "Vec::is_empty", default)]
        pub native_tokens: Vec<NativeToken>,
        pub serial_number: u32,
        pub token_scheme: TokenSchemeDto,
        pub unlock_conditions: Vec<UnlockConditionDto>,
        #[serde(skip_serializing_if = "Vec::is_empty", default)]
        pub features: Vec<FeatureDto>,
        #[serde(skip_serializing_if = "Vec::is_empty", default)]
        pub immutable_features: Vec<FeatureDto>,
    }

    impl From<FoundryOutput> for FoundryOutputDto {
        fn from(value: FoundryOutput) -> Self {
            Self {
                kind: 5,
                amount: value.amount.to_string(),
                native_tokens: value.native_tokens.inner,
                serial_number: value.serial_number,
                token_scheme: value.token_scheme.into(),
                unlock_conditions: value.unlock_conditions.inner.into_iter().map(Into::into).collect(),
                features: value.features.inner.into_iter().map(Into::into).collect(),
                immutable_features: value.immutable_features.inner.into_iter().map(Into::into).collect(),
            }
        }
    }

    #[derive(Deserialize)]
    pub struct NftOutput {
        pub amount: u64,
        pub native_tokens: BoxedSlicePrefix<NativeToken>,
        pub nft_id: Value,
        pub unlock_conditions: BoxedSlicePrefix<UnlockCondition>,
        pub features: BoxedSlicePrefix<Feature>,
        pub immutable_features: BoxedSlicePrefix<Feature>,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct NftOutputDto {
        #[serde(rename = "type")]
        pub kind: u8,
        pub amount: String,
        #[serde(skip_serializing_if = "Vec::is_empty", default)]
        pub native_tokens: Vec<NativeToken>,
        pub nft_id: Value,
        pub unlock_conditions: Vec<UnlockConditionDto>,
        #[serde(skip_serializing_if = "Vec::is_empty", default)]
        pub features: Vec<FeatureDto>,
        #[serde(skip_serializing_if = "Vec::is_empty", default)]
        pub immutable_features: Vec<FeatureDto>,
    }

    impl From<NftOutput> for NftOutputDto {
        fn from(value: NftOutput) -> Self {
            Self {
                kind: 6,
                amount: value.amount.to_string(),
                native_tokens: value.native_tokens.inner,
                nft_id: value.nft_id,
                unlock_conditions: value.unlock_conditions.inner.into_iter().map(Into::into).collect(),
                features: value.features.inner.into_iter().map(Into::into).collect(),
                immutable_features: value.immutable_features.inner.into_iter().map(Into::into).collect(),
            }
        }
    }

    #[derive(Deserialize)]
    #[repr(transparent)]
    pub struct BoxedSlicePrefix<T> {
        pub inner: Vec<T>,
    }

    #[derive(Deserialize)]
    #[serde(tag = "type", content = "data")]
    pub enum UnlockCondition {
        Address(Address),
        StorageDepositReturn(StorageDepositReturnUnlockCondition),
        Timelock(u32),
        Expiration(ExpirationUnlockCondition),
        StateControllerAddress(Address),
        GovernorAddress(Address),
        ImmutableAliasAddress(Address),
    }

    #[derive(Serialize, From)]
    #[serde(untagged)]
    pub enum UnlockConditionDto {
        Address(AddressUnlockConditionDto),
        StorageDepositReturn(StorageDepositReturnUnlockConditionDto),
        Timelock(TimelockUnlockConditionDto),
        Expiration(ExpirationUnlockConditionDto),
        StateControllerAddress(StateControllerAddressUnlockConditionDto),
        GovernorAddress(GovernorAddressUnlockConditionDto),
        ImmutableAliasAddress(ImmutableAliasAddressUnlockConditionDto),
    }

    impl From<UnlockCondition> for UnlockConditionDto {
        fn from(value: UnlockCondition) -> Self {
            match value {
                UnlockCondition::Address(address) => AddressUnlockConditionDto {
                    kind: 0,
                    address: address.into(),
                }
                .into(),
                UnlockCondition::StorageDepositReturn(StorageDepositReturnUnlockCondition {
                    return_address,
                    amount,
                }) => StorageDepositReturnUnlockConditionDto {
                    kind: 1,
                    return_address: return_address.into(),
                    amount: amount.to_string(),
                }
                .into(),
                UnlockCondition::Timelock(timestamp) => TimelockUnlockConditionDto { kind: 2, timestamp }.into(),
                UnlockCondition::Expiration(ExpirationUnlockCondition {
                    return_address,
                    timestamp,
                }) => ExpirationUnlockConditionDto {
                    kind: 3,
                    return_address: return_address.into(),
                    timestamp,
                }
                .into(),
                UnlockCondition::StateControllerAddress(address) => StateControllerAddressUnlockConditionDto {
                    kind: 4,
                    address: address.into(),
                }
                .into(),
                UnlockCondition::GovernorAddress(address) => GovernorAddressUnlockConditionDto {
                    kind: 5,
                    address: address.into(),
                }
                .into(),
                UnlockCondition::ImmutableAliasAddress(address) => ImmutableAliasAddressUnlockConditionDto {
                    kind: 6,
                    address: address.into(),
                }
                .into(),
            }
        }
    }

    impl<'de> Deserialize<'de> for UnlockConditionDto {
        fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            let value = Value::deserialize(d)?;
            Ok(
                match value
                    .get("type")
                    .and_then(Value::as_u64)
                    .ok_or_else(|| serde::de::Error::custom("invalid unlock condition type"))?
                    as u8
                {
                    0 => Self::Address(AddressUnlockConditionDto::deserialize(value).map_err(|e| {
                        serde::de::Error::custom(format!("cannot deserialize address unlock condition: {e}"))
                    })?),
                    1 => Self::StorageDepositReturn(
                        StorageDepositReturnUnlockConditionDto::deserialize(value).map_err(|e| {
                            serde::de::Error::custom(format!(
                                "cannot deserialize storage deposit unlock condition: {e}"
                            ))
                        })?,
                    ),
                    2 => Self::Timelock(TimelockUnlockConditionDto::deserialize(value).map_err(|e| {
                        serde::de::Error::custom(format!("cannot deserialize timelock unlock condition: {e}"))
                    })?),
                    3 => Self::Expiration(ExpirationUnlockConditionDto::deserialize(value).map_err(|e| {
                        serde::de::Error::custom(format!("cannot deserialize expiration unlock condition: {e}"))
                    })?),
                    4 => Self::StateControllerAddress(
                        StateControllerAddressUnlockConditionDto::deserialize(value).map_err(|e| {
                            serde::de::Error::custom(format!(
                                "cannot deserialize state controller unlock condition: {e}"
                            ))
                        })?,
                    ),
                    5 => Self::GovernorAddress(GovernorAddressUnlockConditionDto::deserialize(value).map_err(|e| {
                        serde::de::Error::custom(format!("cannot deserialize governor unlock condition: {e}"))
                    })?),
                    6 => Self::ImmutableAliasAddress(
                        ImmutableAliasAddressUnlockConditionDto::deserialize(value).map_err(|e| {
                            serde::de::Error::custom(format!(
                                "cannot deserialize immutable alias address unlock condition: {e}"
                            ))
                        })?,
                    ),
                    _ => return Err(serde::de::Error::custom("invalid unlock condition type")),
                },
            )
        }
    }

    #[derive(Serialize, Deserialize)]
    pub struct AddressUnlockConditionDto {
        #[serde(rename = "type")]
        pub kind: u8,
        pub address: AddressDto,
    }

    #[derive(Deserialize)]
    pub struct StorageDepositReturnUnlockCondition {
        pub return_address: Address,
        pub amount: u64,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct StorageDepositReturnUnlockConditionDto {
        #[serde(rename = "type")]
        pub kind: u8,
        pub return_address: AddressDto,
        pub amount: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct TimelockUnlockConditionDto {
        #[serde(rename = "type")]
        pub kind: u8,
        #[serde(rename = "unixTime")]
        pub timestamp: u32,
    }

    #[derive(Deserialize)]
    pub struct ExpirationUnlockCondition {
        pub return_address: Address,
        pub timestamp: u32,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ExpirationUnlockConditionDto {
        #[serde(rename = "type")]
        pub kind: u8,
        pub return_address: AddressDto,
        #[serde(rename = "unixTime")]
        pub timestamp: u32,
    }

    #[derive(Serialize, Deserialize)]
    pub struct StateControllerAddressUnlockConditionDto {
        #[serde(rename = "type")]
        pub kind: u8,
        pub address: AddressDto,
    }

    #[derive(Serialize, Deserialize)]
    pub struct GovernorAddressUnlockConditionDto {
        #[serde(rename = "type")]
        pub kind: u8,
        pub address: AddressDto,
    }

    #[derive(Serialize, Deserialize)]
    pub struct ImmutableAliasAddressUnlockConditionDto {
        #[serde(rename = "type")]
        pub kind: u8,
        pub address: AddressDto,
    }

    #[derive(Deserialize)]
    #[serde(tag = "type", content = "data")]
    pub enum Address {
        Ed25519(String),
        Alias(String),
        Nft(String),
    }

    #[derive(Serialize, From)]
    #[serde(untagged)]
    pub enum AddressDto {
        Ed25519(Ed25519AddressDto),
        Alias(AliasAddressDto),
        Nft(NftAddressDto),
    }

    impl From<Address> for AddressDto {
        fn from(value: Address) -> Self {
            match value {
                Address::Ed25519(pub_key_hash) => Ed25519AddressDto { kind: 0, pub_key_hash }.into(),
                Address::Alias(alias_id) => AliasAddressDto { kind: 8, alias_id }.into(),
                Address::Nft(nft_id) => NftAddressDto { kind: 16, nft_id }.into(),
            }
        }
    }

    impl<'de> Deserialize<'de> for AddressDto {
        fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            let value = Value::deserialize(d)?;
            Ok(
                match value
                    .get("type")
                    .and_then(Value::as_u64)
                    .ok_or_else(|| serde::de::Error::custom("invalid address type"))? as u8
                {
                    0 => {
                        Self::Ed25519(Ed25519AddressDto::deserialize(value).map_err(|e| {
                            serde::de::Error::custom(format!("cannot deserialize ed25519 address: {e}"))
                        })?)
                    }
                    8 => Self::Alias(
                        AliasAddressDto::deserialize(value)
                            .map_err(|e| serde::de::Error::custom(format!("cannot deserialize alias address: {e}")))?,
                    ),
                    16 => Self::Nft(
                        NftAddressDto::deserialize(value)
                            .map_err(|e| serde::de::Error::custom(format!("cannot deserialize NFT address: {e}")))?,
                    ),
                    _ => return Err(serde::de::Error::custom("invalid address type")),
                },
            )
        }
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Ed25519AddressDto {
        #[serde(rename = "type")]
        pub kind: u8,
        pub pub_key_hash: String,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct AliasAddressDto {
        #[serde(rename = "type")]
        pub kind: u8,
        pub alias_id: String,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct NftAddressDto {
        #[serde(rename = "type")]
        pub kind: u8,
        pub nft_id: String,
    }

    #[derive(Deserialize)]
    #[serde(tag = "type", content = "data")]
    pub enum Feature {
        Sender(Address),
        Issuer(Address),
        Metadata(BoxedSlicePrefix<u8>),
        Tag(BoxedSlicePrefix<u8>),
    }

    #[derive(Serialize, From)]
    #[serde(untagged)]
    pub enum FeatureDto {
        Sender(SenderFeatureDto),
        Issuer(IssuerFeatureDto),
        Metadata(MetadataFeatureDto),
        Tag(TagFeatureDto),
    }

    impl From<Feature> for FeatureDto {
        fn from(value: Feature) -> Self {
            match value {
                Feature::Sender(address) => SenderFeatureDto {
                    kind: 0,
                    address: address.into(),
                }
                .into(),
                Feature::Issuer(address) => IssuerFeatureDto {
                    kind: 1,
                    address: address.into(),
                }
                .into(),
                Feature::Metadata(data) => MetadataFeatureDto {
                    kind: 2,
                    data: prefix_hex::encode(data.inner),
                }
                .into(),
                Feature::Tag(tag) => TagFeatureDto {
                    kind: 3,
                    tag: prefix_hex::encode(tag.inner),
                }
                .into(),
            }
        }
    }

    impl<'de> Deserialize<'de> for FeatureDto {
        fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            let value = Value::deserialize(d)?;
            Ok(
                match value
                    .get("type")
                    .and_then(Value::as_u64)
                    .ok_or_else(|| serde::de::Error::custom("invalid feature type"))? as u8
                {
                    0 => Self::Sender(
                        SenderFeatureDto::deserialize(value)
                            .map_err(|e| serde::de::Error::custom(format!("cannot deserialize sender feature: {e}")))?,
                    ),
                    1 => Self::Issuer(
                        IssuerFeatureDto::deserialize(value)
                            .map_err(|e| serde::de::Error::custom(format!("cannot deserialize issuer feature: {e}")))?,
                    ),
                    2 => {
                        Self::Metadata(MetadataFeatureDto::deserialize(value).map_err(|e| {
                            serde::de::Error::custom(format!("cannot deserialize metadata feature: {e}"))
                        })?)
                    }
                    3 => Self::Tag(
                        TagFeatureDto::deserialize(value)
                            .map_err(|e| serde::de::Error::custom(format!("cannot deserialize tag feature: {e}")))?,
                    ),
                    _ => return Err(serde::de::Error::custom("invalid feature type")),
                },
            )
        }
    }

    #[derive(Serialize, Deserialize)]
    pub struct SenderFeatureDto {
        #[serde(rename = "type")]
        pub kind: u8,
        pub address: AddressDto,
    }

    #[derive(Serialize, Deserialize)]
    pub struct IssuerFeatureDto {
        #[serde(rename = "type")]
        pub kind: u8,
        pub address: AddressDto,
    }

    #[derive(Serialize, Deserialize)]
    pub struct MetadataFeatureDto {
        #[serde(rename = "type")]
        pub kind: u8,
        pub data: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct TagFeatureDto {
        #[serde(rename = "type")]
        pub kind: u8,
        pub tag: String,
    }

    #[derive(Deserialize)]
    pub enum TokenScheme {
        Simple(SimpleTokenScheme),
    }

    #[derive(Serialize, Deserialize, From)]
    #[serde(untagged)]
    pub enum TokenSchemeDto {
        /// A simple token scheme.
        Simple(SimpleTokenSchemeDto),
    }

    impl From<TokenScheme> for TokenSchemeDto {
        fn from(value: TokenScheme) -> Self {
            match value {
                TokenScheme::Simple(s) => SimpleTokenSchemeDto::from(s).into(),
            }
        }
    }

    #[derive(Deserialize)]
    pub struct SimpleTokenScheme {
        pub minted_tokens: Value,
        pub melted_tokens: Value,
        pub maximum_supply: Value,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SimpleTokenSchemeDto {
        #[serde(rename = "type")]
        pub kind: u8,
        pub minted_tokens: Value,
        pub melted_tokens: Value,
        pub maximum_supply: Value,
    }

    impl From<SimpleTokenScheme> for SimpleTokenSchemeDto {
        fn from(value: SimpleTokenScheme) -> Self {
            Self {
                kind: 0,
                minted_tokens: value.minted_tokens,
                melted_tokens: value.melted_tokens,
                maximum_supply: value.maximum_supply,
            }
        }
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Transaction {
        pub payload: TransactionPayload,
        #[serde(default)]
        pub block_id: Option<Value>,
        pub inclusion_state: Value,
        pub timestamp: u128,
        pub transaction_id: Value,
        pub network_id: u64,
        pub incoming: bool,
        #[serde(default)]
        pub note: Option<String>,
        #[serde(default)]
        pub inputs: Vec<Value>,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct TransactionDto {
        pub payload: TransactionPayloadDto,
        pub block_id: Option<Value>,
        pub inclusion_state: Value,
        pub timestamp: String,
        pub transaction_id: Value,
        pub network_id: String,
        pub incoming: bool,
        pub note: Option<String>,
        pub inputs: Vec<Value>,
    }

    #[derive(Deserialize)]
    pub struct TransactionPayload {
        pub essence: TransactionEssence,
        pub unlocks: BoxedSlicePrefix<Unlock>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct TransactionPayloadDto {
        #[serde(rename = "type")]
        pub kind: u32,
        pub essence: TransactionEssenceDto,
        pub unlocks: Vec<UnlockDto>,
    }

    impl From<TransactionPayload> for TransactionPayloadDto {
        fn from(value: TransactionPayload) -> Self {
            Self {
                kind: 6,
                essence: match value.essence {
                    TransactionEssence::Regular(essence) => RegularTransactionEssenceDto::from(essence).into(),
                },
                unlocks: value.unlocks.inner.into_iter().map(Into::into).collect(),
            }
        }
    }

    #[derive(Deserialize)]
    #[serde(tag = "type", content = "data")]
    pub enum TransactionEssence {
        Regular(RegularTransactionEssence),
    }

    #[derive(Serialize, Deserialize, From)]
    #[serde(untagged)]
    pub enum TransactionEssenceDto {
        Regular(RegularTransactionEssenceDto),
    }

    #[derive(Deserialize)]
    pub struct RegularTransactionEssence {
        network_id: u64,
        inputs: BoxedSlicePrefix<Input>,
        inputs_commitment: Vec<u8>,
        outputs: BoxedSlicePrefix<Output>,
        payload: Option<Payload>,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct RegularTransactionEssenceDto {
        #[serde(rename = "type")]
        pub kind: u8,
        pub network_id: String,
        pub inputs: Vec<InputDto>,
        pub inputs_commitment: String,
        pub outputs: Vec<OutputDto>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub payload: Option<TaggedDataPayloadDto>,
    }

    impl From<RegularTransactionEssence> for RegularTransactionEssenceDto {
        fn from(value: RegularTransactionEssence) -> Self {
            Self {
                kind: 1,
                network_id: value.network_id.to_string(),
                inputs: value.inputs.inner.into_iter().map(Into::into).collect(),
                inputs_commitment: prefix_hex::encode(value.inputs_commitment),
                outputs: value.outputs.inner.into_iter().map(Into::into).collect(),
                payload: value.payload.map(|p| {
                    let Payload::TaggedData(payload) = p;
                    payload.into()
                }),
            }
        }
    }

    #[derive(Deserialize)]
    #[serde(tag = "type", content = "data")]
    pub enum Payload {
        TaggedData(TaggedDataPayload),
    }

    #[derive(Deserialize)]
    pub struct TaggedDataPayload {
        tag: BoxedSlicePrefix<u8>,
        data: BoxedSlicePrefix<u8>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct TaggedDataPayloadDto {
        #[serde(rename = "type")]
        pub kind: u32,
        #[serde(skip_serializing_if = "String::is_empty", default)]
        pub tag: String,
        #[serde(skip_serializing_if = "String::is_empty", default)]
        pub data: String,
    }

    impl From<TaggedDataPayload> for TaggedDataPayloadDto {
        fn from(value: TaggedDataPayload) -> Self {
            Self {
                kind: 5,
                tag: prefix_hex::encode(value.tag.inner),
                data: prefix_hex::encode(value.data.inner),
            }
        }
    }

    #[derive(Deserialize)]
    #[serde(tag = "type", content = "data")]
    pub enum Input {
        Utxo(OutputId),
        Treasury(String),
    }

    #[derive(Serialize, Deserialize, From)]
    #[serde(untagged)]
    pub enum InputDto {
        Utxo(UtxoInputDto),
        Treasury(TreasuryInputDto),
    }

    impl From<Input> for InputDto {
        fn from(value: Input) -> Self {
            match value {
                Input::Utxo(OutputId { transaction_id, index }) => UtxoInputDto {
                    kind: 0,
                    transaction_id: transaction_id.to_string(),
                    transaction_output_index: index,
                }
                .into(),
                Input::Treasury(milestone_id) => TreasuryInputDto { kind: 1, milestone_id }.into(),
            }
        }
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct UtxoInputDto {
        #[serde(rename = "type")]
        pub kind: u8,
        pub transaction_id: String,
        pub transaction_output_index: u16,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct TreasuryInputDto {
        #[serde(rename = "type")]
        pub kind: u8,
        pub milestone_id: String,
    }

    #[derive(Deserialize)]
    #[serde(tag = "type", content = "data")]
    pub enum Unlock {
        Signature(Signature),
        Reference(u16),
        Alias(u16),
        Nft(u16),
    }

    #[derive(Serialize, From)]
    #[serde(untagged)]
    pub enum UnlockDto {
        Signature(SignatureUnlockDto),
        Reference(ReferenceUnlockDto),
        Alias(AliasUnlockDto),
        Nft(NftUnlockDto),
    }

    impl From<Unlock> for UnlockDto {
        fn from(value: Unlock) -> Self {
            match value {
                Unlock::Signature(signature) => {
                    let Signature::Ed25519(signature) = signature;
                    SignatureUnlockDto {
                        kind: 0,
                        signature: signature.into(),
                    }
                    .into()
                }
                Unlock::Reference(index) => ReferenceUnlockDto { kind: 1, index }.into(),
                Unlock::Alias(index) => AliasUnlockDto { kind: 2, index }.into(),
                Unlock::Nft(index) => NftUnlockDto { kind: 3, index }.into(),
            }
        }
    }

    impl<'de> Deserialize<'de> for UnlockDto {
        fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            let value = Value::deserialize(d)?;
            Ok(
                match value
                    .get("type")
                    .and_then(Value::as_u64)
                    .ok_or_else(|| serde::de::Error::custom("invalid unlock type"))? as u8
                {
                    0 => {
                        Self::Signature(SignatureUnlockDto::deserialize(value).map_err(|e| {
                            serde::de::Error::custom(format!("cannot deserialize signature unlock: {e}"))
                        })?)
                    }
                    1 => {
                        Self::Reference(ReferenceUnlockDto::deserialize(value).map_err(|e| {
                            serde::de::Error::custom(format!("cannot deserialize reference unlock: {e}"))
                        })?)
                    }
                    2 => Self::Alias(
                        AliasUnlockDto::deserialize(value)
                            .map_err(|e| serde::de::Error::custom(format!("cannot deserialize alias unlock: {e}")))?,
                    ),
                    3 => Self::Nft(
                        NftUnlockDto::deserialize(value)
                            .map_err(|e| serde::de::Error::custom(format!("cannot deserialize NFT unlock: {e}")))?,
                    ),
                    _ => return Err(serde::de::Error::custom("invalid unlock type")),
                },
            )
        }
    }

    #[derive(Serialize, Deserialize)]
    pub struct SignatureUnlockDto {
        #[serde(rename = "type")]
        pub kind: u8,
        pub signature: Ed25519SignatureDto,
    }

    #[derive(Serialize, Deserialize)]
    pub struct ReferenceUnlockDto {
        #[serde(rename = "type")]
        pub kind: u8,
        #[serde(rename = "reference")]
        pub index: u16,
    }

    #[derive(Serialize, Deserialize)]
    pub struct AliasUnlockDto {
        #[serde(rename = "type")]
        pub kind: u8,
        #[serde(rename = "reference")]
        pub index: u16,
    }

    #[derive(Serialize, Deserialize)]
    pub struct NftUnlockDto {
        #[serde(rename = "type")]
        pub kind: u8,
        #[serde(rename = "reference")]
        pub index: u16,
    }

    #[derive(Deserialize)]
    #[serde(tag = "type", content = "data")]
    pub enum Signature {
        Ed25519(Ed25519Signature),
    }

    #[derive(Deserialize)]
    pub struct Ed25519Signature {
        pub public_key: Vec<u8>,
        pub signature: Vec<u8>,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Ed25519SignatureDto {
        #[serde(rename = "type")]
        pub kind: u8,
        pub public_key: String,
        pub signature: String,
    }

    impl From<Ed25519Signature> for Ed25519SignatureDto {
        fn from(value: Ed25519Signature) -> Self {
            Self {
                kind: 0,
                public_key: prefix_hex::encode(value.public_key),
                signature: prefix_hex::encode(value.signature),
            }
        }
    }
}

struct ConvertNetworkName;
impl Convert for ConvertNetworkName {
    type New = String;
    type Old = super::migrate_1::types::StringPrefix;

    fn convert(old: Self::Old) -> crate::wallet::Result<Self::New> {
        Ok(old.inner)
    }
}

struct ConvertTokenSupply;
impl Convert for ConvertTokenSupply {
    type New = String;
    type Old = u64;

    fn convert(old: Self::Old) -> crate::wallet::Result<Self::New> {
        Ok(old.to_string())
    }
}

struct ConvertOutputData;
impl Convert for ConvertOutputData {
    type New = types::OutputDataDto;
    type Old = types::OutputData;

    fn convert(old: Self::Old) -> crate::wallet::Result<Self::New> {
        Ok(Self::New {
            output_id: old.output_id,
            metadata: migrate_0::types::OutputMetadataDto {
                block_id: old.metadata.block_id,
                transaction_id: old.metadata.output_id.transaction_id.to_string(),
                output_index: old.metadata.output_id.index,
                is_spent: old.metadata.is_spent,
                milestone_index_spent: old.metadata.milestone_index_spent,
                milestone_timestamp_spent: old.metadata.milestone_timestamp_spent,
                transaction_id_spent: old.metadata.transaction_id_spent.as_ref().map(ToString::to_string),
                milestone_index_booked: old.metadata.milestone_index_booked,
                milestone_timestamp_booked: old.metadata.milestone_timestamp_booked,
                ledger_index: old.metadata.ledger_index,
            },
            output: old.output.into(),
            is_spent: old.is_spent,
            address: old.address.into(),
            network_id: old.network_id.to_string(),
            remainder: old.remainder,
            chain: old.chain,
        })
    }
}

struct ConvertTransaction;
impl Convert for ConvertTransaction {
    type New = types::TransactionDto;
    type Old = types::Transaction;

    fn convert(old: Self::Old) -> crate::wallet::Result<Self::New> {
        Ok(Self::New {
            payload: old.payload.into(),
            block_id: old.block_id,
            inclusion_state: old.inclusion_state,
            timestamp: old.timestamp.to_string(),
            transaction_id: old.transaction_id,
            network_id: old.network_id.to_string(),
            incoming: old.incoming,
            note: old.note,
            inputs: old.inputs,
        })
    }
}

struct ConvertFoundryOutput;
impl Convert for ConvertFoundryOutput {
    type New = types::FoundryOutputDto;
    type Old = types::FoundryOutput;

    fn convert(old: Self::Old) -> crate::wallet::Result<Self::New> {
        Ok(old.into())
    }
}
