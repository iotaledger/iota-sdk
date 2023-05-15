// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::str::FromStr;
use std::collections::HashMap;

use serde::de::DeserializeOwned;

use super::*;
use crate::wallet::Error;

pub struct Migrate;

#[async_trait]
impl Migration for Migrate {
    const ID: usize = 0;
    const SDK_VERSION: &'static str = "0.4.0";
    const DATE: time::Date = time::macros::date!(2023 - 05 - 15);

    #[cfg(feature = "storage")]
    async fn migrate_storage(storage: &crate::wallet::storage::Storage) -> Result<()> {
        use crate::wallet::storage::constants::{ACCOUNTS_INDEXATION_KEY, ACCOUNT_INDEXATION_KEY};

        if let Some(account_indexes) = storage.get::<Vec<u32>>(ACCOUNTS_INDEXATION_KEY).await? {
            for account_index in account_indexes {
                if let Some(mut account) = storage
                    .get::<serde_json::Value>(&format!("{ACCOUNT_INDEXATION_KEY}{account_index}"))
                    .await?
                {
                    ConvertIncomingTransactions::check(
                        account
                            .get_mut("incomingTransactions")
                            .ok_or(Error::Storage("missing incoming transactions".to_owned()))?,
                    )?;
                    for output_data in account
                        .get_mut("outputs")
                        .ok_or(Error::Storage("missing outputs".to_owned()))?
                        .as_object_mut()
                        .ok_or(Error::Storage("malformatted outputs".to_owned()))?
                        .values_mut()
                    {
                        ConvertOutputMetadata::check(
                            output_data
                                .get_mut("metadata")
                                .ok_or(Error::Storage("missing metadata".to_owned()))?,
                        )?;
                    }
                    for output_data in account
                        .get_mut("unspentOutputs")
                        .ok_or(Error::Storage("missing unspent outputs".to_owned()))?
                        .as_object_mut()
                        .ok_or(Error::Storage("malformatted unspent outputs".to_owned()))?
                        .values_mut()
                    {
                        ConvertOutputMetadata::check(
                            output_data
                                .get_mut("metadata")
                                .ok_or(Error::Storage("missing metadata".to_owned()))?,
                        )?;
                    }
                    storage
                        .set(&format!("{ACCOUNT_INDEXATION_KEY}{account_index}"), account)
                        .await?;
                }
            }
        }
        Ok(())
    }

    #[cfg(feature = "stronghold")]
    async fn migrate_backup(storage: &crate::client::stronghold::StrongholdAdapter) -> Result<()> {
        use crate::{
            client::storage::StorageProvider,
            wallet::wallet::operations::stronghold_backup::stronghold_snapshot::ACCOUNTS_KEY,
        };

        if let Some(mut accounts) = storage
            .get(ACCOUNTS_KEY.as_bytes())
            .await?
            .map(|bytes| serde_json::from_slice::<Vec<serde_json::Value>>(&bytes))
            .transpose()?
        {
            for account in &mut accounts {
                ConvertIncomingTransactions::check(
                    account
                        .get_mut("incomingTransactions")
                        .ok_or(Error::Storage("missing incoming transactions".to_owned()))?,
                )?;
                for output_data in account
                    .get_mut("outputs")
                    .ok_or(Error::Storage("missing outputs".to_owned()))?
                    .as_object_mut()
                    .ok_or(Error::Storage("malformatted outputs".to_owned()))?
                    .values_mut()
                {
                    ConvertOutputMetadata::check(
                        output_data
                            .get_mut("metadata")
                            .ok_or(Error::Storage("missing metadata".to_owned()))?,
                    )?;
                }
                for output_data in account
                    .get_mut("unspentOutputs")
                    .ok_or(Error::Storage("missing unspent outputs".to_owned()))?
                    .as_object_mut()
                    .ok_or(Error::Storage("malformatted unspent outputs".to_owned()))?
                    .values_mut()
                {
                    ConvertOutputMetadata::check(
                        output_data
                            .get_mut("metadata")
                            .ok_or(Error::Storage("missing metadata".to_owned()))?,
                    )?;
                }
            }
            storage
                .insert(ACCOUNTS_KEY.as_bytes(), serde_json::to_string(&accounts)?.as_bytes())
                .await?;
        }
        storage.delete(b"backup_schema_version").await.ok();
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
    use serde::{Deserialize, Serialize};

    use crate::{impl_id, string_serde_impl};

    impl_id!(
        pub TransactionId,
        32,
        "A transaction identifier, the BLAKE2b-256 hash of the transaction bytes. See <https://www.blake2.net/> for more information."
    );

    #[cfg(feature = "serde")]
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

    #[derive(Serialize, Deserialize)]
    pub struct OutputId {
        pub transaction_id: TransactionId,
        pub index: serde_json::Value,
    }

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
        pub output_index: serde_json::Value,
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
