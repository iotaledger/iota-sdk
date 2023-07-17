// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::*;

pub struct Migrate;

fn migrate_account(account: &mut serde_json::Value) -> Result<()> {
    for output_data in account["outputs"]
        .as_object_mut()
        .ok_or(Error::Storage("malformatted outputs".to_owned()))?
        .values_mut()
    {
        if let Some(chain) = output_data.get_mut("chain") {
            ConvertChain::check(chain)?;
        }
    }

    for output_data in account["unspentOutputs"]
        .as_object_mut()
        .ok_or(Error::Storage("malformatted unspent outputs".to_owned()))?
        .values_mut()
    {
        if let Some(chain) = output_data.get_mut("chain") {
            ConvertChain::check(chain)?;
        }
    }
    Ok(())
}

#[async_trait]
impl MigrationData for Migrate {
    const ID: usize = 2;
    const SDK_VERSION: &'static str = "0.4.0";
    const DATE: time::Date = time::macros::date!(2023 - 07 - 14);
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
        Ok(())
    }
}

mod types {
    use serde::{Deserialize, Serialize};

    pub const HARDEN_MASK: u32 = 1 << 31;

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Bip44 {
        pub coin_type: u32,
        pub account: u32,
        pub change: u32,
        pub address_index: u32,
    }
}

struct ConvertChain;
impl Convert for ConvertChain {
    type New = types::Bip44;
    type Old = [u32; 5];

    fn convert(old: Self::Old) -> crate::wallet::Result<Self::New> {
        Ok(Self::New {
            coin_type: old[1] & !types::HARDEN_MASK,
            account: old[2] & !types::HARDEN_MASK,
            change: old[3] & !types::HARDEN_MASK,
            address_index: old[4] & !types::HARDEN_MASK,
        })
    }
}
