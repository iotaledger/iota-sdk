// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

use serde::de::DeserializeOwned;

use super::*;
use crate::{
    types::{
        api::core::response::OutputWithMetadataResponse,
        block::{
            output::{dto::OutputMetadataDto, OutputMetadata},
            payload::{transaction::TransactionId, TransactionPayload},
        },
    },
    wallet::{
        account::{build_transaction_from_payload_and_inputs, types::Transaction},
        Error,
    },
};
// use crate::types::block::address::Hrp;
// use packable::prefix::StringPrefix;

pub struct Migrate;

#[async_trait]
impl Migration for Migrate {
    const ID: usize = 0;
    const SDK_VERSION: &'static str = "0.3.0";
    const DATE: time::Date = time::macros::date!(2023 - 05 - 09);

    #[cfg(feature = "storage")]
    async fn migrate_storage(storage: &crate::wallet::storage::Storage) -> Result<()> {
        use crate::wallet::{
            account::AccountDetails,
            storage::constants::{ACCOUNTS_INDEXATION_KEY, ACCOUNT_INDEXATION_KEY},
        };

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
                        .set(
                            &format!("{ACCOUNT_INDEXATION_KEY}{account_index}"),
                            serde_json::from_value::<AccountDetails>(account)?,
                        )
                        .await?;
                }
            }
        }

        // if let Some(mut wallet) = storage.get::<serde_json::Value>(WALLET_INDEXATION_KEY).await? {
        //     ConvertHrp::check(
        //         wallet
        //             .get_mut("clientOptions")
        //             .ok_or(Error::Storage("missing client options".to_owned()))?
        //             .get_mut("protocolParameters")
        //             .ok_or(Error::Storage("missing protocol params".to_owned()))?
        //             .get_mut("bech32Hrp")
        //             .ok_or(Error::Storage("missing bech32 hrp".to_owned()))?,
        //     )?;
        //     let wallet_builder = serde_json::from_value::<WalletBuilder>(wallet.clone())?;
        //     storage.save_wallet_data(&wallet_builder).await?;
        // }
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
        Ok(())
    }
}

trait Convert {
    type New: Serialize + DeserializeOwned;
    type Old: DeserializeOwned;

    fn check(value: &mut serde_json::Value) -> crate::wallet::Result<()> {
        if serde_json::from_value::<Self::New>(value.clone()).is_err() {
            *value = serde_json::to_value(Self::convert(serde_json::from_value::<Self::Old>(value.clone())?))?;
        }
        Ok(())
    }

    fn convert(old: Self::Old) -> crate::wallet::Result<Self::New>;
}

struct ConvertIncomingTransactions;
impl Convert for ConvertIncomingTransactions {
    type New = HashMap<TransactionId, Transaction>;
    type Old = HashMap<TransactionId, (TransactionPayload, Vec<OutputWithMetadataResponse>)>;

    fn convert(old: Self::Old) -> crate::wallet::Result<Self::New> {
        let mut new = HashMap::new();
        for (tx_id, (tx_payload, inputs)) in old {
            new.insert(
                tx_id,
                build_transaction_from_payload_and_inputs(tx_id, tx_payload, inputs)?,
            );
        }
        Ok(new)
    }
}

struct ConvertOutputMetadata;
impl Convert for ConvertOutputMetadata {
    type New = OutputMetadata;
    type Old = OutputMetadataDto;

    fn convert(old: Self::Old) -> crate::wallet::Result<Self::New> {
        Ok(Self::New::try_from(&old)?)
    }
}

// struct ConvertHrp;
// impl Convert for ConvertHrp {
//     type New = Hrp;
//     type Old = StringPrefix<u8>;

//     fn convert(old: Self::Old) -> crate::wallet::Result<Self::New> {
//         Ok(Self::New::from_str_unchecked(old.as_str()))
//     }
// }
