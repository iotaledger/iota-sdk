// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::*;

pub(crate) struct Migrate;

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
            if let Some(client_options) = wallet.get_mut("client_options") {
                let params = client_options["protocolParameters"].as_object_mut().unwrap();
                if let Some(version) = params.remove("protocol_version") {
                    params.insert("version".to_owned(), version);
                }
                ConvertNetworkName::check(&mut client_options["protocolParameters"]["network_name"])?;
                ConvertTokenSupply::check(&mut client_options["protocolParameters"]["token_supply"])?;
            }
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
    pub(crate) struct OutputData {
        pub(crate) output_id: Value,
        pub(crate) metadata: OutputMetadata,
        pub(crate) output: Output,
        pub(crate) is_spent: bool,
        pub(crate) address: Address,
        pub(crate) network_id: u64,
        pub(crate) remainder: bool,
        pub(crate) chain: Option<Bip44>,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub(crate) struct OutputDataDto {
        pub(crate) output_id: Value,
        pub(crate) metadata: OutputMetadataDto,
        pub(crate) output: OutputDto,
        pub(crate) is_spent: bool,
        pub(crate) address: AddressDto,
        pub(crate) network_id: String,
        pub(crate) remainder: bool,
        pub(crate) chain: Option<Bip44>,
    }

    #[derive(Deserialize)]
    #[serde(tag = "type", content = "data")]
    pub(crate) enum Output {
        Treasury(TreasuryOutput),
        Basic(BasicOutput),
        Alias(AliasOutput),
        Foundry(FoundryOutput),
        Nft(NftOutput),
    }

    #[derive(Serialize, From)]
    #[serde(untagged)]
    pub(crate) enum OutputDto {
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
    pub(crate) struct TreasuryOutput {
        pub(crate) amount: u64,
    }

    #[derive(Serialize, Deserialize)]
    pub(crate) struct TreasuryOutputDto {
        #[serde(rename = "type")]
        pub(crate) kind: u8,
        pub(crate) amount: String,
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
    pub(crate) struct BasicOutput {
        pub(crate) amount: u64,
        pub(crate) native_tokens: BoxedSlicePrefix<NativeToken>,
        pub(crate) unlock_conditions: BoxedSlicePrefix<UnlockCondition>,
        pub(crate) features: BoxedSlicePrefix<Feature>,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub(crate) struct BasicOutputDto {
        #[serde(rename = "type")]
        pub(crate) kind: u8,
        pub(crate) amount: String,
        #[serde(skip_serializing_if = "Vec::is_empty", default)]
        pub(crate) native_tokens: Vec<NativeToken>,
        pub(crate) unlock_conditions: Vec<UnlockConditionDto>,
        #[serde(skip_serializing_if = "Vec::is_empty", default)]
        pub(crate) features: Vec<FeatureDto>,
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
    pub(crate) struct AliasOutput {
        pub(crate) amount: u64,
        pub(crate) native_tokens: BoxedSlicePrefix<NativeToken>,
        pub(crate) alias_id: Value,
        pub(crate) state_index: u32,
        pub(crate) state_metadata: BoxedSlicePrefix<u8>,
        pub(crate) foundry_counter: u32,
        pub(crate) unlock_conditions: BoxedSlicePrefix<UnlockCondition>,
        pub(crate) features: BoxedSlicePrefix<Feature>,
        pub(crate) immutable_features: BoxedSlicePrefix<Feature>,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub(crate) struct AliasOutputDto {
        #[serde(rename = "type")]
        pub(crate) kind: u8,
        pub(crate) amount: String,
        #[serde(skip_serializing_if = "Vec::is_empty", default)]
        pub(crate) native_tokens: Vec<NativeToken>,
        pub(crate) alias_id: Value,
        pub(crate) state_index: u32,
        #[serde(skip_serializing_if = "String::is_empty", default)]
        pub(crate) state_metadata: String,
        pub(crate) foundry_counter: u32,
        pub(crate) unlock_conditions: Vec<UnlockConditionDto>,
        #[serde(skip_serializing_if = "Vec::is_empty", default)]
        pub(crate) features: Vec<FeatureDto>,
        #[serde(skip_serializing_if = "Vec::is_empty", default)]
        pub(crate) immutable_features: Vec<FeatureDto>,
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
    pub(crate) struct FoundryOutput {
        pub(crate) amount: u64,
        pub(crate) native_tokens: BoxedSlicePrefix<NativeToken>,
        pub(crate) serial_number: u32,
        pub(crate) token_scheme: TokenScheme,
        pub(crate) unlock_conditions: BoxedSlicePrefix<UnlockCondition>,
        pub(crate) features: BoxedSlicePrefix<Feature>,
        pub(crate) immutable_features: BoxedSlicePrefix<Feature>,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub(crate) struct FoundryOutputDto {
        #[serde(rename = "type")]
        pub(crate) kind: u8,
        pub(crate) amount: String,
        #[serde(skip_serializing_if = "Vec::is_empty", default)]
        pub(crate) native_tokens: Vec<NativeToken>,
        pub(crate) serial_number: u32,
        pub(crate) token_scheme: TokenSchemeDto,
        pub(crate) unlock_conditions: Vec<UnlockConditionDto>,
        #[serde(skip_serializing_if = "Vec::is_empty", default)]
        pub(crate) features: Vec<FeatureDto>,
        #[serde(skip_serializing_if = "Vec::is_empty", default)]
        pub(crate) immutable_features: Vec<FeatureDto>,
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
    pub(crate) struct NftOutput {
        pub(crate) amount: u64,
        pub(crate) native_tokens: BoxedSlicePrefix<NativeToken>,
        pub(crate) nft_id: Value,
        pub(crate) unlock_conditions: BoxedSlicePrefix<UnlockCondition>,
        pub(crate) features: BoxedSlicePrefix<Feature>,
        pub(crate) immutable_features: BoxedSlicePrefix<Feature>,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub(crate) struct NftOutputDto {
        #[serde(rename = "type")]
        pub(crate) kind: u8,
        pub(crate) amount: String,
        #[serde(skip_serializing_if = "Vec::is_empty", default)]
        pub(crate) native_tokens: Vec<NativeToken>,
        pub(crate) nft_id: Value,
        pub(crate) unlock_conditions: Vec<UnlockConditionDto>,
        #[serde(skip_serializing_if = "Vec::is_empty", default)]
        pub(crate) features: Vec<FeatureDto>,
        #[serde(skip_serializing_if = "Vec::is_empty", default)]
        pub(crate) immutable_features: Vec<FeatureDto>,
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
    pub(crate) struct BoxedSlicePrefix<T> {
        pub(crate) inner: Vec<T>,
    }

    #[derive(Deserialize)]
    #[serde(tag = "type", content = "data")]
    pub(crate) enum UnlockCondition {
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
    pub(crate) enum UnlockConditionDto {
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
    pub(crate) struct AddressUnlockConditionDto {
        #[serde(rename = "type")]
        pub(crate) kind: u8,
        pub(crate) address: AddressDto,
    }

    #[derive(Deserialize)]
    pub(crate) struct StorageDepositReturnUnlockCondition {
        pub(crate) return_address: Address,
        pub(crate) amount: u64,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub(crate) struct StorageDepositReturnUnlockConditionDto {
        #[serde(rename = "type")]
        pub(crate) kind: u8,
        pub(crate) return_address: AddressDto,
        pub(crate) amount: String,
    }

    #[derive(Serialize, Deserialize)]
    pub(crate) struct TimelockUnlockConditionDto {
        #[serde(rename = "type")]
        pub(crate) kind: u8,
        #[serde(rename = "unixTime")]
        pub(crate) timestamp: u32,
    }

    #[derive(Deserialize)]
    pub(crate) struct ExpirationUnlockCondition {
        pub(crate) return_address: Address,
        pub(crate) timestamp: u32,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub(crate) struct ExpirationUnlockConditionDto {
        #[serde(rename = "type")]
        pub(crate) kind: u8,
        pub(crate) return_address: AddressDto,
        #[serde(rename = "unixTime")]
        pub(crate) timestamp: u32,
    }

    #[derive(Serialize, Deserialize)]
    pub(crate) struct StateControllerAddressUnlockConditionDto {
        #[serde(rename = "type")]
        pub(crate) kind: u8,
        pub(crate) address: AddressDto,
    }

    #[derive(Serialize, Deserialize)]
    pub(crate) struct GovernorAddressUnlockConditionDto {
        #[serde(rename = "type")]
        pub(crate) kind: u8,
        pub(crate) address: AddressDto,
    }

    #[derive(Serialize, Deserialize)]
    pub(crate) struct ImmutableAliasAddressUnlockConditionDto {
        #[serde(rename = "type")]
        pub(crate) kind: u8,
        pub(crate) address: AddressDto,
    }

    #[derive(Deserialize)]
    #[serde(tag = "type", content = "data")]
    pub(crate) enum Address {
        Ed25519(String),
        Alias(String),
        Nft(String),
    }

    #[derive(Serialize, From)]
    #[serde(untagged)]
    pub(crate) enum AddressDto {
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
    pub(crate) struct Ed25519AddressDto {
        #[serde(rename = "type")]
        pub(crate) kind: u8,
        pub(crate) pub_key_hash: String,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub(crate) struct AliasAddressDto {
        #[serde(rename = "type")]
        pub(crate) kind: u8,
        pub(crate) alias_id: String,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub(crate) struct NftAddressDto {
        #[serde(rename = "type")]
        pub(crate) kind: u8,
        pub(crate) nft_id: String,
    }

    #[derive(Deserialize)]
    #[serde(tag = "type", content = "data")]
    pub(crate) enum Feature {
        Sender(Address),
        Issuer(Address),
        Metadata(BoxedSlicePrefix<u8>),
        Tag(BoxedSlicePrefix<u8>),
    }

    #[derive(Serialize, From)]
    #[serde(untagged)]
    pub(crate) enum FeatureDto {
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
    pub(crate) struct SenderFeatureDto {
        #[serde(rename = "type")]
        pub(crate) kind: u8,
        pub(crate) address: AddressDto,
    }

    #[derive(Serialize, Deserialize)]
    pub(crate) struct IssuerFeatureDto {
        #[serde(rename = "type")]
        pub(crate) kind: u8,
        pub(crate) address: AddressDto,
    }

    #[derive(Serialize, Deserialize)]
    pub(crate) struct MetadataFeatureDto {
        #[serde(rename = "type")]
        pub(crate) kind: u8,
        pub(crate) data: String,
    }

    #[derive(Serialize, Deserialize)]
    pub(crate) struct TagFeatureDto {
        #[serde(rename = "type")]
        pub(crate) kind: u8,
        pub(crate) tag: String,
    }

    #[derive(Deserialize)]
    pub(crate) enum TokenScheme {
        Simple(SimpleTokenScheme),
    }

    #[derive(Serialize, Deserialize, From)]
    #[serde(untagged)]
    pub(crate) enum TokenSchemeDto {
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
    pub(crate) struct SimpleTokenScheme {
        pub(crate) minted_tokens: Value,
        pub(crate) melted_tokens: Value,
        pub(crate) maximum_supply: Value,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub(crate) struct SimpleTokenSchemeDto {
        #[serde(rename = "type")]
        pub(crate) kind: u8,
        pub(crate) minted_tokens: Value,
        pub(crate) melted_tokens: Value,
        pub(crate) maximum_supply: Value,
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
    pub(crate) struct Transaction {
        pub(crate) payload: TransactionPayload,
        #[serde(default)]
        pub(crate) block_id: Option<Value>,
        pub(crate) inclusion_state: Value,
        pub(crate) timestamp: u128,
        pub(crate) transaction_id: Value,
        pub(crate) network_id: u64,
        pub(crate) incoming: bool,
        #[serde(default)]
        pub(crate) note: Option<String>,
        #[serde(default)]
        pub(crate) inputs: Vec<Value>,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub(crate) struct TransactionDto {
        pub(crate) payload: TransactionPayloadDto,
        pub(crate) block_id: Option<Value>,
        pub(crate) inclusion_state: Value,
        pub(crate) timestamp: String,
        pub(crate) transaction_id: Value,
        pub(crate) network_id: String,
        pub(crate) incoming: bool,
        pub(crate) note: Option<String>,
        pub(crate) inputs: Vec<Value>,
    }

    #[derive(Deserialize)]
    pub(crate) struct TransactionPayload {
        pub(crate) essence: TransactionEssence,
        pub(crate) unlocks: BoxedSlicePrefix<Unlock>,
    }

    #[derive(Serialize, Deserialize)]
    pub(crate) struct TransactionPayloadDto {
        #[serde(rename = "type")]
        pub(crate) kind: u32,
        pub(crate) essence: TransactionEssenceDto,
        pub(crate) unlocks: Vec<UnlockDto>,
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
    pub(crate) enum TransactionEssence {
        Regular(RegularTransactionEssence),
    }

    #[derive(Serialize, Deserialize, From)]
    #[serde(untagged)]
    pub(crate) enum TransactionEssenceDto {
        Regular(RegularTransactionEssenceDto),
    }

    #[derive(Deserialize)]
    pub(crate) struct RegularTransactionEssence {
        network_id: u64,
        inputs: BoxedSlicePrefix<Input>,
        inputs_commitment: Vec<u8>,
        outputs: BoxedSlicePrefix<Output>,
        payload: Option<Payload>,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub(crate) struct RegularTransactionEssenceDto {
        #[serde(rename = "type")]
        pub(crate) kind: u8,
        pub(crate) network_id: String,
        pub(crate) inputs: Vec<InputDto>,
        pub(crate) inputs_commitment: String,
        pub(crate) outputs: Vec<OutputDto>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub(crate) payload: Option<TaggedDataPayloadDto>,
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
    pub(crate) enum Payload {
        TaggedData(TaggedDataPayload),
    }

    #[derive(Deserialize)]
    pub(crate) struct TaggedDataPayload {
        tag: BoxedSlicePrefix<u8>,
        data: BoxedSlicePrefix<u8>,
    }

    #[derive(Serialize, Deserialize)]
    pub(crate) struct TaggedDataPayloadDto {
        #[serde(rename = "type")]
        pub(crate) kind: u32,
        #[serde(skip_serializing_if = "String::is_empty", default)]
        pub(crate) tag: String,
        #[serde(skip_serializing_if = "String::is_empty", default)]
        pub(crate) data: String,
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
    pub(crate) enum Input {
        Utxo(OutputId),
        Treasury(String),
    }

    #[derive(Serialize, Deserialize, From)]
    #[serde(untagged)]
    pub(crate) enum InputDto {
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
    pub(crate) struct UtxoInputDto {
        #[serde(rename = "type")]
        pub(crate) kind: u8,
        pub(crate) transaction_id: String,
        pub(crate) transaction_output_index: u16,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub(crate) struct TreasuryInputDto {
        #[serde(rename = "type")]
        pub(crate) kind: u8,
        pub(crate) milestone_id: String,
    }

    #[derive(Deserialize)]
    #[serde(tag = "type", content = "data")]
    pub(crate) enum Unlock {
        Signature(Signature),
        Reference(u16),
        Alias(u16),
        Nft(u16),
    }

    #[derive(Serialize, From)]
    #[serde(untagged)]
    pub(crate) enum UnlockDto {
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
    pub(crate) struct SignatureUnlockDto {
        #[serde(rename = "type")]
        pub(crate) kind: u8,
        pub(crate) signature: Ed25519SignatureDto,
    }

    #[derive(Serialize, Deserialize)]
    pub(crate) struct ReferenceUnlockDto {
        #[serde(rename = "type")]
        pub(crate) kind: u8,
        #[serde(rename = "reference")]
        pub(crate) index: u16,
    }

    #[derive(Serialize, Deserialize)]
    pub(crate) struct AliasUnlockDto {
        #[serde(rename = "type")]
        pub(crate) kind: u8,
        #[serde(rename = "reference")]
        pub(crate) index: u16,
    }

    #[derive(Serialize, Deserialize)]
    pub(crate) struct NftUnlockDto {
        #[serde(rename = "type")]
        pub(crate) kind: u8,
        #[serde(rename = "reference")]
        pub(crate) index: u16,
    }

    #[derive(Deserialize)]
    #[serde(tag = "type", content = "data")]
    pub(crate) enum Signature {
        Ed25519(Ed25519Signature),
    }

    #[derive(Deserialize)]
    pub(crate) struct Ed25519Signature {
        pub(crate) public_key: Vec<u8>,
        pub(crate) signature: Vec<u8>,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub(crate) struct Ed25519SignatureDto {
        #[serde(rename = "type")]
        pub(crate) kind: u8,
        pub(crate) public_key: String,
        pub(crate) signature: String,
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
