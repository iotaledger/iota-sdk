// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::str::FromStr;
use std::collections::HashMap;

use super::*;
use crate::wallet::Error;

pub struct Migrate;

fn migrate_account(account: &mut serde_json::Value) -> Result<()> {
    for output_data in account["outputs"]
        .as_object_mut()
        .ok_or(Error::Storage("malformatted outputs".to_owned()))?
        .values_mut()
    {
        ConvertOutputMetadata::check(&mut output_data["metadata"])?;
    }

    for output_data in account["unspentOutputs"]
        .as_object_mut()
        .ok_or(Error::Storage("malformatted unspent outputs".to_owned()))?
        .values_mut()
    {
        ConvertOutputMetadata::check(&mut output_data["metadata"])?;
    }

    ConvertIncomingTransactions::check(&mut account["incomingTransactions"])?;

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
        use crate::wallet::storage::constants::{ACCOUNTS_INDEXATION_KEY, ACCOUNT_INDEXATION_KEY};

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

        Ok(())
    }
}

#[async_trait]
#[cfg(feature = "stronghold")]
impl Migration<crate::client::stronghold::StrongholdAdapter> for Migrate {
    async fn migrate(storage: &crate::client::stronghold::StrongholdAdapter) -> Result<()> {
        use crate::{
            client::storage::StorageAdapter,
            wallet::core::operations::stronghold_backup::stronghold_snapshot::ACCOUNTS_KEY,
        };

        if let Some(mut accounts) = storage.get::<Vec<serde_json::Value>>(ACCOUNTS_KEY).await? {
            for account in &mut accounts {
                migrate_account(account)?;
            }
            storage.set(ACCOUNTS_KEY, &accounts).await?;
        }
        storage.delete("backup_schema_version").await.ok();
        Ok(())
    }
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
