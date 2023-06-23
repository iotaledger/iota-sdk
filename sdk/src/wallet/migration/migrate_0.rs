// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::{marker::PhantomData, str::FromStr};
use std::collections::HashMap;

use serde::de::DeserializeOwned;
use serde_json::json;

use super::*;
use crate::wallet::Error;

pub struct Migrate;

fn rename_keys(json: &mut serde_json::Value) {
    match json {
        serde_json::Value::Array(a) => a.iter_mut().for_each(rename_keys),
        serde_json::Value::Object(o) => {
            let mut replace = serde_json::Map::with_capacity(o.len());
            o.retain(|k, v| {
                rename_keys(v);
                replace.insert(
                    heck::ToLowerCamelCase::to_lower_camel_case(k.as_str()),
                    std::mem::replace(v, serde_json::Value::Null),
                );
                true
            });
            *o = replace;
        }
        _ => (),
    }
}

fn migrate_native_tokens(native_tokens: &mut serde_json::Value) {
    let mut native_tokens_v = native_tokens.take();
    *native_tokens = native_tokens_v["inner"].take();
    for native_token in native_tokens.as_array_mut().unwrap() {
        if let Some(id) = native_token.get("token_id") {
            *native_token = serde_json::json!({ "amount": native_token["amount"], "id": id});
        }
    }
}

fn migrate_token_scheme(token_scheme: &mut serde_json::Value) {
    let token_scheme_map = token_scheme.as_object_mut().unwrap();
    if let Some(mut data) = token_scheme_map.remove("Simple") {
        rename_keys(&mut data);
        *token_scheme = json!({
            "type": 0,
            "data": data
        });
    }
}

fn migrate_token_scheme_dto(token_scheme: &mut serde_json::Value) {
    let mut token_scheme_v = token_scheme.take();
    let token_scheme_map = token_scheme_v.as_object_mut().unwrap();
    let kind = token_scheme_map.remove("type");
    *token_scheme = json!({
        "type": kind,
        "data": token_scheme_v
    });
}

fn migrate_address(address: &mut serde_json::Value) -> Result<()> {
    ConvertTag::<types::AddressTag>::check(&mut address["type"])?;
    Ok(())
}

fn migrate_address_dto(address: &mut serde_json::Value) -> Result<()> {
    let address = address.as_object_mut().unwrap();
    if let Some(pub_key_hash) = address.remove("pubKeyHash") {
        address.insert("data".to_owned(), pub_key_hash);
    }
    if let Some(alias_id) = address.remove("aliasId") {
        address.insert("data".to_owned(), alias_id);
    }
    if let Some(nft_id) = address.remove("nftId") {
        address.insert("data".to_owned(), nft_id);
    }
    Ok(())
}

fn migrate_unlock_conditions(unlock_conditions: &mut serde_json::Value) -> Result<()> {
    let mut unlock_conditions_v = unlock_conditions.take();
    *unlock_conditions = unlock_conditions_v["inner"].take();
    for unlock_condition in unlock_conditions.as_array_mut().unwrap() {
        ConvertTag::<types::UnlockConditionTag>::check(&mut unlock_condition["type"])?;
        if let Some(kind) = unlock_condition["type"].as_u64() {
            match kind {
                // Address
                // Governor
                // ImmutableAlias
                // StateController
                // Here data is an address
                0 | 4 | 5 | 6 => migrate_address(&mut unlock_condition["data"])?,
                // StorageDepositReturn
                // Expiration
                // These contain an address
                1 | 3 => {
                    migrate_address(&mut unlock_condition["data"]["return_address"])?;
                    rename_keys(&mut unlock_condition["data"]);
                }
                _ => (),
            }
        }
    }
    Ok(())
}

fn migrate_unlock_conditions_dto(unlock_conditions: &mut serde_json::Value) -> Result<()> {
    for unlock_condition in unlock_conditions.as_array_mut().unwrap() {
        if let Some(kind) = unlock_condition["type"].as_u64() {
            match kind {
                // Address
                // Governor
                // ImmutableAlias
                // StateController
                // These should be renamed as they are a single field
                0 | 4 | 5 | 6 => {
                    let unlock_condition = unlock_condition.as_object_mut().unwrap();
                    if let Some(mut address) = unlock_condition.remove("address") {
                        migrate_address_dto(&mut address)?;
                        unlock_condition.insert("data".to_owned(), address);
                    }
                }
                // StorageDepositReturn
                // Timelock
                // Expiration
                // These are flattened objects so we have to reorganize
                1 | 2 | 3 => {
                    let mut unlock_condition_v = unlock_condition.take();
                    if let Some(return_address) = unlock_condition_v.get_mut("returnAddress") {
                        migrate_address_dto(return_address)?;
                    }
                    let unlock_condition_map = unlock_condition_v.as_object_mut().unwrap();
                    unlock_condition_map.remove("type");
                    *unlock_condition = json!({
                        "type": kind,
                        "data": unlock_condition_v
                    });
                }
                _ => (),
            }
        }
    }
    Ok(())
}

fn migrate_features(features: &mut serde_json::Value) -> Result<()> {
    let mut features_v = features.take();
    *features = features_v["inner"].take();
    for feature in features.as_array_mut().unwrap() {
        ConvertTag::<types::FeatureTag>::check(&mut feature["type"])?;
        if let Some(kind) = feature["type"].as_u64() {
            match kind {
                0 | 1 => migrate_address(&mut feature["data"])?,
                2 | 3 => ConvertBoxedSlice::check(&mut feature["data"])?,
                _ => (),
            }
        }
    }
    Ok(())
}

fn migrate_features_dto(features: &mut serde_json::Value) -> Result<()> {
    for feature in features.as_array_mut().unwrap() {
        let feature = feature.as_object_mut().unwrap();
        if let Some(mut address) = feature.remove("address") {
            migrate_address_dto(&mut address)?;
            feature.insert("data".to_owned(), address);
        }
        if let Some(tag) = feature.remove("tag") {
            feature.insert("data".to_owned(), tag);
        }
    }
    Ok(())
}

fn migrate_output_inner(output: &mut serde_json::Value) -> Result<()> {
    if let Some(amount) = output.get_mut("amount") {
        *amount = amount.as_u64().unwrap().to_string().into();
    }
    if let Some(state_metadata) = output.get_mut("state_metadata") {
        ConvertBoxedSlice::check(state_metadata)?
    }
    if let Some(features) = output.get_mut("features") {
        migrate_features(features)?;
    }
    if let Some(features) = output.get_mut("immutable_features") {
        migrate_features(features)?;
    }
    if let Some(native_tokens) = output.get_mut("native_tokens") {
        migrate_native_tokens(native_tokens);
    }
    if let Some(unlock_conditions) = output.get_mut("unlock_conditions") {
        migrate_unlock_conditions(unlock_conditions)?;
    }
    if let Some(token_scheme) = output.get_mut("token_scheme") {
        migrate_token_scheme(token_scheme);
    }
    rename_keys(output);
    Ok(())
}

fn migrate_output(output: &mut serde_json::Value) -> Result<()> {
    ConvertTag::<types::OutputTag>::check(&mut output["type"])?;
    migrate_output_inner(&mut output["data"])?;
    Ok(())
}

fn migrate_output_dto(output: &mut serde_json::Value) -> Result<()> {
    let mut output_v = output.take();
    if let Some(state_metadata) = output.get_mut("stateMetadata") {
        ConvertBoxedSlice::check(state_metadata)?
    }
    if let Some(features) = output_v.get_mut("features") {
        migrate_features_dto(features)?;
    }
    if let Some(features) = output_v.get_mut("immutableFeatures") {
        migrate_features_dto(features)?;
    }
    if let Some(unlock_conditions) = output_v.get_mut("unlockConditions") {
        migrate_unlock_conditions_dto(unlock_conditions)?;
    }
    if let Some(token_scheme) = output_v.get_mut("tokenScheme") {
        migrate_token_scheme_dto(token_scheme);
    }
    let output_map = output_v.as_object_mut().unwrap();
    let kind = output_map.remove("type");
    *output = json!({
        "type": kind,
        "data": output_v
    });
    Ok(())
}

fn migrate_output_data(output_data: &mut serde_json::Value) -> Result<()> {
    ConvertOutputMetadata::check(&mut output_data["metadata"])?;
    migrate_address(&mut output_data["address"])?;

    migrate_output(&mut output_data["output"])?;

    if let Some(chain) = output_data.get_mut("chain").and_then(|c| c.as_array_mut()) {
        for segment in chain {
            ConvertSegment::check(segment)?;
        }
    }
    Ok(())
}

fn migrate_transactions_map(map: &mut serde_json::Value) -> Result<()> {
    for (_key, transaction) in map.as_object_mut().unwrap() {
        for input in transaction["inputs"].as_array_mut().unwrap() {
            migrate_output_dto(&mut input["output"])?;
            ConvertOutputMetadata::check(&mut input["metadata"])?;
        }

        let mut unlocks_v = transaction["payload"]["unlocks"].take();
        transaction["payload"]["unlocks"] = unlocks_v["inner"].take();

        let outputs = transaction["payload"]["essence"]["data"]["outputs"]["inner"]
            .as_array_mut()
            .unwrap();
        for output in outputs {
            migrate_output(output)?;
        }
    }
    Ok(())
}

fn migrate_account(account: &mut serde_json::Value) -> Result<()> {
    migrate_transactions_map(&mut account["transactions"])?;

    ConvertIncomingTransactions::check(&mut account["incomingTransactions"])?;

    migrate_transactions_map(&mut account["incomingTransactions"])?;

    if let Some(native_token_foundries) = account.get_mut("nativeTokenFoundries") {
        for (_key, foundry) in native_token_foundries.as_object_mut().unwrap() {
            migrate_output_inner(foundry)?;
        }
    }

    for output_data in account["outputs"]
        .as_object_mut()
        .ok_or(Error::Storage("malformatted outputs".to_owned()))?
        .values_mut()
    {
        migrate_output_data(output_data)?;
    }

    for output_data in account["unspentOutputs"]
        .as_object_mut()
        .ok_or(Error::Storage("malformatted unspent outputs".to_owned()))?
        .values_mut()
    {
        migrate_output_data(output_data)?;
    }

    Ok(())
}

fn migrate_client_options(client_options: &mut serde_json::Value) -> Result<()> {
    let protocol_parameters = &mut client_options["protocolParameters"];

    ConvertHrp::check(&mut protocol_parameters["bech32_hrp"])?;

    // TODO this is temporary to merge https://github.com/iotaledger/iota-sdk/pull/570.
    // We actually need to migrate the whole protocol_parameters, including this.
    rename_keys(&mut protocol_parameters["rent_structure"]);

    Ok(())
}

#[async_trait]
impl MigrationData for Migrate {
    const ID: usize = 0;
    const SDK_VERSION: &'static str = "0.4.0";
    const DATE: time::Date = time::macros::date!(2023 - 06 - 14);
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
            migrate_client_options(&mut wallet["client_options"])?;

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
            wallet::wallet::operations::stronghold_backup::stronghold_snapshot::{ACCOUNTS_KEY, CLIENT_OPTIONS_KEY},
        };

        if let Some(mut accounts) = storage.get::<Vec<serde_json::Value>>(ACCOUNTS_KEY).await? {
            for account in &mut accounts {
                migrate_account(account)?;
            }
            storage.set(ACCOUNTS_KEY, &accounts).await?;
        }
        if let Some(mut client_options) = storage.get::<serde_json::Value>(CLIENT_OPTIONS_KEY).await? {
            migrate_client_options(&mut client_options)?;

            storage.set(CLIENT_OPTIONS_KEY, &client_options).await?;
        }
        storage.delete("backup_schema_version").await.ok();
        Ok(())
    }
}

trait Convert {
    type New: Serialize + DeserializeOwned;
    type Old: DeserializeOwned;

    fn check(value: &mut serde_json::Value) -> crate::wallet::Result<()> {
        if serde_json::from_value::<Self::New>(value.clone()).is_err() {
            *value = serde_json::to_value(Self::convert(serde_json::from_value::<Self::Old>(value.clone())?)?)?;
        }
        Ok(())
    }

    fn convert(old: Self::Old) -> crate::wallet::Result<Self::New>;
}

mod types {
    use core::str::FromStr;

    use serde::{Deserialize, Serialize};

    use crate::types::block::Error;

    macro_rules! string_serde_impl {
        ($type:ty) => {
            impl serde::Serialize for $type {
                fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
                    use alloc::string::ToString;

                    s.serialize_str(&self.to_string())
                }
            }

            impl<'de> serde::Deserialize<'de> for $type {
                fn deserialize<D>(deserializer: D) -> Result<$type, D::Error>
                where
                    D: serde::Deserializer<'de>,
                {
                    struct StringVisitor;

                    impl<'de> serde::de::Visitor<'de> for StringVisitor {
                        type Value = $type;

                        fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                            formatter.write_str("a string representing the value")
                        }

                        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                        where
                            E: serde::de::Error,
                        {
                            let value = core::str::FromStr::from_str(v).map_err(serde::de::Error::custom)?;
                            Ok(value)
                        }
                    }

                    deserializer.deserialize_str(StringVisitor)
                }
            }
        };
    }

    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct TransactionId([u8; Self::LENGTH]);

    impl TransactionId {
        pub const LENGTH: usize = 32;
    }

    impl core::str::FromStr for TransactionId {
        type Err = Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(Self(prefix_hex::decode(s).map_err(Error::Hex)?))
        }
    }

    impl core::fmt::Display for TransactionId {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "{}", prefix_hex::encode(self.0))
        }
    }

    string_serde_impl!(TransactionId);

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Transaction {
        pub payload: TransactionPayload,
        pub block_id: Option<serde_json::Value>,
        pub inclusion_state: InclusionState,
        pub timestamp: u128,
        pub transaction_id: TransactionId,
        pub network_id: u64,
        pub incoming: bool,
        pub note: Option<String>,
        #[serde(default)]
        pub inputs: Vec<OutputWithMetadataResponse>,
    }

    #[derive(Serialize, Deserialize)]
    pub struct TransactionPayload {
        pub essence: TransactionEssence,
        pub unlocks: serde_json::Value,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(tag = "type", content = "data")]
    pub enum TransactionEssence {
        Regular(RegularTransactionEssence),
    }

    #[derive(Serialize, Deserialize)]
    pub struct RegularTransactionEssence {
        pub network_id: u64,
        pub inputs: serde_json::Value,
        pub inputs_commitment: serde_json::Value,
        pub outputs: serde_json::Value,
        pub payload: serde_json::Value,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct OutputWithMetadataResponse {
        pub metadata: OutputMetadataDto,
        pub output: serde_json::Value,
    }

    pub struct OutputId {
        pub transaction_id: TransactionId,
        pub index: u16,
    }

    impl OutputId {
        pub const LENGTH: usize = TransactionId::LENGTH + core::mem::size_of::<u16>();
    }

    impl TryFrom<[u8; Self::LENGTH]> for OutputId {
        type Error = Error;

        fn try_from(bytes: [u8; Self::LENGTH]) -> Result<Self, Self::Error> {
            let (transaction_id, index) = bytes.split_at(TransactionId::LENGTH);

            Ok(Self {
                transaction_id: TransactionId(transaction_id.try_into().unwrap()),
                index: u16::from_le_bytes(index.try_into().unwrap()),
            })
        }
    }

    impl FromStr for OutputId {
        type Err = Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Self::try_from(prefix_hex::decode::<[u8; Self::LENGTH]>(s).map_err(Error::Hex)?)
        }
    }

    impl core::fmt::Display for OutputId {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            let mut buffer = [0u8; Self::LENGTH];
            let (transaction_id, index) = buffer.split_at_mut(TransactionId::LENGTH);
            transaction_id.copy_from_slice(&self.transaction_id.0);
            index.copy_from_slice(&self.index.to_le_bytes());
            write!(f, "{}", prefix_hex::encode(buffer))
        }
    }

    string_serde_impl!(OutputId);

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct OutputMetadata {
        pub block_id: serde_json::Value,
        pub output_id: OutputId,
        pub is_spent: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub milestone_index_spent: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub milestone_timestamp_spent: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub transaction_id_spent: Option<TransactionId>,
        pub milestone_index_booked: u32,
        pub milestone_timestamp_booked: u32,
        pub ledger_index: u32,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct OutputMetadataDto {
        pub block_id: serde_json::Value,
        pub transaction_id: String,
        pub output_index: u16,
        pub is_spent: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub milestone_index_spent: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub milestone_timestamp_spent: Option<u32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub transaction_id_spent: Option<String>,
        pub milestone_index_booked: u32,
        pub milestone_timestamp_booked: u32,
        pub ledger_index: u32,
    }

    #[derive(Serialize, Deserialize)]
    pub enum InclusionState {
        Pending,
        Confirmed,
        Conflicting,
        UnknownPruned,
    }

    #[derive(Deserialize)]
    #[allow(non_camel_case_types)]
    pub struct Crypto_0_18_0_Segment {
        pub bs: [u8; 4],
        pub hardened: bool,
    }

    pub struct Hrp {
        inner: [u8; 83],
        len: u8,
    }

    impl Hrp {
        /// Convert a string to an Hrp without checking validity.
        pub const fn from_str_unchecked(hrp: &str) -> Self {
            let len = hrp.len();
            let mut bytes = [0; 83];
            let hrp = hrp.as_bytes();
            let mut i = 0;
            while i < len {
                bytes[i] = hrp[i];
                i += 1;
            }
            Self {
                inner: bytes,
                len: len as _,
            }
        }
    }

    impl FromStr for Hrp {
        type Err = Error;

        fn from_str(hrp: &str) -> Result<Self, Self::Err> {
            let len = hrp.len();
            if hrp.is_ascii() && len <= 83 {
                let mut bytes = [0; 83];
                bytes[..len].copy_from_slice(hrp.as_bytes());
                Ok(Self {
                    inner: bytes,
                    len: len as _,
                })
            } else {
                Err(Error::InvalidBech32Hrp(hrp.to_string()))
            }
        }
    }

    impl core::fmt::Display for Hrp {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            let hrp_str = self.inner[..self.len as usize]
                .iter()
                .map(|b| *b as char)
                .collect::<String>();
            f.write_str(&hrp_str)
        }
    }

    string_serde_impl!(Hrp);

    #[derive(Serialize, Deserialize)]
    #[repr(transparent)]
    pub struct StringPrefix {
        pub inner: String,
        bounded: (),
    }

    #[derive(Serialize, Deserialize)]
    #[repr(transparent)]
    pub struct BoxedSlicePrefix<T> {
        pub inner: Box<[T]>,
        bounded: (),
    }

    #[derive(Deserialize)]
    #[repr(u8)]
    pub enum FeatureTag {
        Sender,
        Issuer,
        Metadata,
        Tag,
    }

    #[derive(Deserialize)]
    #[repr(u8)]
    pub enum UnlockConditionTag {
        Address,
        StorageDepositReturn,
        Timelock,
        Expiration,
        StateControllerAddress,
        GovernorAddress,
        ImmutableAliasAddress,
    }

    #[derive(Deserialize)]
    #[repr(u8)]
    pub enum AddressTag {
        Ed25519 = 0,
        Alias = 8,
        Nft = 16,
    }

    #[derive(Deserialize)]
    #[repr(u8)]
    pub enum OutputTag {
        Treasury = 2,
        Basic = 3,
        Alias = 4,
        Foundry = 5,
        Nft = 6,
    }
    macro_rules! impl_as_u8 {
        ($v:ident) => {
            impl From<$v> for u8 {
                fn from(value: $v) -> Self {
                    value as _
                }
            }
        };
    }
    impl_as_u8!(FeatureTag);
    impl_as_u8!(UnlockConditionTag);
    impl_as_u8!(AddressTag);
    impl_as_u8!(OutputTag);
}

struct ConvertIncomingTransactions;
impl Convert for ConvertIncomingTransactions {
    type New = HashMap<types::TransactionId, types::Transaction>;
    type Old = HashMap<types::TransactionId, (types::TransactionPayload, Vec<types::OutputWithMetadataResponse>)>;

    fn convert(old: Self::Old) -> crate::wallet::Result<Self::New> {
        let mut new = HashMap::new();
        for (tx_id, (tx_payload, inputs)) in old {
            let types::TransactionEssence::Regular(tx_essence) = &tx_payload.essence;
            let txn = types::Transaction {
                network_id: tx_essence.network_id,
                payload: tx_payload,
                block_id: inputs
                    .first()
                    .map(|i: &types::OutputWithMetadataResponse| i.metadata.block_id.clone()),
                inclusion_state: types::InclusionState::Confirmed,
                timestamp: inputs
                    .first()
                    .and_then(|i| i.metadata.milestone_timestamp_spent.map(|t| t as u128 * 1000))
                    .unwrap_or_else(|| crate::utils::unix_timestamp_now().as_millis()),
                transaction_id: tx_id,
                incoming: true,
                note: None,
                inputs,
            };
            new.insert(tx_id, txn);
        }
        Ok(new)
    }
}

struct ConvertOutputMetadata;
impl Convert for ConvertOutputMetadata {
    type New = types::OutputMetadata;
    type Old = types::OutputMetadataDto;

    fn convert(old: Self::Old) -> crate::wallet::Result<Self::New> {
        Ok(Self::New {
            block_id: old.block_id,
            output_id: types::OutputId {
                transaction_id: types::TransactionId::from_str(&old.transaction_id)?,
                index: old.output_index,
            },
            is_spent: old.is_spent,
            milestone_index_spent: old.milestone_index_spent,
            milestone_timestamp_spent: old.milestone_timestamp_spent,
            transaction_id_spent: old
                .transaction_id_spent
                .as_ref()
                .map(|s| types::TransactionId::from_str(s))
                .transpose()?,
            milestone_index_booked: old.milestone_index_booked,
            milestone_timestamp_booked: old.milestone_timestamp_booked,
            ledger_index: old.ledger_index,
        })
    }
}

struct ConvertSegment;
impl Convert for ConvertSegment {
    type New = u32;
    type Old = types::Crypto_0_18_0_Segment;

    fn convert(old: Self::Old) -> crate::wallet::Result<Self::New> {
        Ok(u32::from_be_bytes(old.bs))
    }
}

struct ConvertHrp;
impl Convert for ConvertHrp {
    type New = types::Hrp;
    type Old = types::StringPrefix;

    fn convert(old: Self::Old) -> crate::wallet::Result<Self::New> {
        Ok(Self::New::from_str_unchecked(&old.inner))
    }
}

struct ConvertTag<Old>(PhantomData<Old>);
impl<Old: Into<u8> + DeserializeOwned> Convert for ConvertTag<Old> {
    type New = u8;
    type Old = Old;

    fn convert(old: Self::Old) -> crate::wallet::Result<Self::New> {
        Ok(old.into())
    }
}

struct ConvertBoxedSlice;
impl Convert for ConvertBoxedSlice {
    type New = String;
    type Old = types::BoxedSlicePrefix<u8>;

    fn convert(old: Self::Old) -> crate::wallet::Result<Self::New> {
        Ok(prefix_hex::encode(old.inner))
    }
}
