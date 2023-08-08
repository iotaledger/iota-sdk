// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{collections::HashMap, path::Path, sync::atomic::Ordering};

use crate::{
    client::{
        constants::IOTA_COIN_TYPE,
        secret::SecretManagerConfig,
        storage::StorageAdapter,
        stronghold::{StrongholdAdapter, PRIVATE_DATA_CLIENT_PATH},
    },
    types::TryFromDto,
    wallet::{
        account::{AccountDetails, AccountDetailsDto},
        migration::{
            chrysalis::{migrate_from_chrysalis_data, CHRYSALIS_STORAGE_KEY},
            latest_backup_migration_version, migrate, MigrationVersion, MIGRATION_VERSION_KEY,
        },
        storage::constants::{ACCOUNTS_INDEXATION_KEY, ACCOUNT_INDEXATION_KEY, WALLET_INDEXATION_KEY},
        ClientOptions, Wallet,
    },
};

pub(crate) const CLIENT_OPTIONS_KEY: &str = "client_options";
pub(crate) const COIN_TYPE_KEY: &str = "coin_type";
pub(crate) const SECRET_MANAGER_KEY: &str = "secret_manager";
pub(crate) const ACCOUNTS_KEY: &str = "accounts";

impl<S: 'static + SecretManagerConfig> Wallet<S> {
    pub(crate) async fn store_data_to_stronghold(&self, stronghold: &StrongholdAdapter) -> crate::wallet::Result<()> {
        // Set migration version
        stronghold
            .set(MIGRATION_VERSION_KEY, &latest_backup_migration_version())
            .await?;

        let client_options = self.client_options().await;
        stronghold.set(CLIENT_OPTIONS_KEY, &client_options).await?;

        let coin_type = self.coin_type.load(Ordering::Relaxed);
        stronghold.set_bytes(COIN_TYPE_KEY, &coin_type.to_le_bytes()).await?;

        if let Some(secret_manager_dto) = self.secret_manager.read().await.to_config() {
            stronghold.set(SECRET_MANAGER_KEY, &secret_manager_dto).await?;
        }

        let mut serialized_accounts = Vec::new();
        for account in self.accounts.read().await.iter() {
            serialized_accounts.push(serde_json::to_value(&AccountDetailsDto::from(
                &*account.details().await,
            ))?);
        }

        stronghold.set(ACCOUNTS_KEY, &serialized_accounts).await?;

        Ok(())
    }
}

pub(crate) async fn read_data_from_stronghold_snapshot<S: 'static + SecretManagerConfig>(
    stronghold: &StrongholdAdapter,
) -> crate::wallet::Result<(
    Option<ClientOptions>,
    Option<u32>,
    Option<S::Config>,
    Option<Vec<AccountDetails>>,
)> {
    // TODO: decide if we should have the chrysalis migration also in the migrate function
    migrate_snapshot_from_chrysalis_to_stardust(stronghold).await?;

    migrate(stronghold).await?;

    // Get client_options
    let client_options = stronghold.get(CLIENT_OPTIONS_KEY).await?;

    // Get coin_type
    let coin_type_bytes = stronghold.get_bytes(COIN_TYPE_KEY).await?;
    let coin_type = if let Some(coin_type_bytes) = coin_type_bytes {
        let coin_type = u32::from_le_bytes(
            coin_type_bytes
                .try_into()
                .map_err(|_| crate::wallet::Error::Backup("invalid coin_type"))?,
        );
        log::debug!("[restore_backup] restored coin_type: {coin_type}");
        Some(coin_type)
    } else {
        None
    };

    // Get secret_manager
    let restored_secret_manager = stronghold.get(SECRET_MANAGER_KEY).await?;

    // Get accounts
    let restored_accounts = stronghold
        .get::<Vec<AccountDetailsDto>>(ACCOUNTS_KEY)
        .await?
        .map(|v| {
            v.into_iter()
                .map(AccountDetails::try_from_dto)
                .collect::<Result<Vec<_>, _>>()
        })
        .transpose()?;

    Ok((client_options, coin_type, restored_secret_manager, restored_accounts))
}

async fn migrate_snapshot_from_chrysalis_to_stardust(
    stronghold_adapter: &StrongholdAdapter,
) -> crate::wallet::Result<()> {
    log::debug!("migrate_snapshot_from_chrysalis_to_stardust");
    let stronghold = stronghold_adapter.inner().await;
    let stronghold_client = stronghold
        .get_client(PRIVATE_DATA_CLIENT_PATH)
        .map_err(|e| crate::wallet::Error::Client(Box::new(crate::client::Error::Stronghold(e.into()))))?;
    let stronghold_store = stronghold_client.store();
    // TODO: find out why there are no keys, different client needed or was there a breaking change in the store?
    let keys = stronghold_store
        .keys()
        .map_err(|e| crate::wallet::Error::Client(Box::new(crate::client::Error::Stronghold(e.into()))))?;
    println!("{keys:?}");
    println!("as bytes: {:?}", "iota-wallet-account-indexation".as_bytes());
    // check if snapshot contains chrysalis data
    if !keys.iter().any(|k| k == "iota-wallet-account-indexation".as_bytes()) {
        return Ok(());
    }
    // TODO: are there snapshots with chrysalis AND stardust data? From shimmer claiming for example
    // What to do with them? Following would also move the stardust account data in the chrysalis data

    let mut chrysalis_data: HashMap<String, String> = HashMap::new();
    for key in keys {
        let value = stronghold_store
            .get(&key)
            .map_err(|e| crate::wallet::Error::Client(Box::new(crate::client::Error::Stronghold(e.into()))))?;

        let key_utf8 = String::from_utf8(key).map_err(|_| crate::wallet::Error::Migration("invalid utf8".into()))?;
        let value_utf8 =
            String::from_utf8(value.unwrap()).map_err(|_| crate::wallet::Error::Migration("invalid utf8".into()))?;

        chrysalis_data.insert(key_utf8, value_utf8);
    }
    println!("{chrysalis_data:?}");
    drop(stronghold_store);
    drop(stronghold_client);
    drop(stronghold);

    let (new_accounts, secret_manager_dto) =
        migrate_from_chrysalis_data(&chrysalis_data, &Path::new("wallet.stronghold"))?;
    // store chrysalis data in a new key
    stronghold_adapter
        .set_bytes(
            CHRYSALIS_STORAGE_KEY,
            serde_json::to_string(&chrysalis_data)?.as_bytes(),
        )
        .await?;

    // write new accounts to db (with account indexation)
    let accounts_indexation_data: Vec<u32> = new_accounts.iter().map(|account| account.index).collect();
    stronghold_adapter
        .set_bytes(
            ACCOUNTS_INDEXATION_KEY,
            serde_json::to_string(&accounts_indexation_data)?.as_bytes(),
        )
        .await?;
    // TODO: do we need to validate the address indexes to be sure that there are no gaps? And if there are, just ignore
    // all above a gap?
    for new_account in new_accounts {
        println!("new_account: {new_account:?}");
        stronghold_adapter
            .set_bytes(
                &format!("{ACCOUNT_INDEXATION_KEY}{}", new_account.index),
                serde_json::to_string(&new_account)?.as_bytes(),
            )
            .await?;
    }

    if let Some(secret_manager_dto) = secret_manager_dto {
        // This is required for the secret manager to be loaded
        stronghold_adapter
            .set_bytes(
                WALLET_INDEXATION_KEY,
                format!("{{ \"coinType\": {IOTA_COIN_TYPE}}}").as_bytes(),
            )
            .await?;
        stronghold_adapter
            .set_bytes(SECRET_MANAGER_KEY, secret_manager_dto.as_bytes())
            .await?;
    }

    // set db migration version
    let migration_version = MigrationVersion {
        id: 4,
        sdk_version: "1.0.0-rc.0".to_string(),
        date: time::macros::date!(2023 - 07 - 19),
    };
    stronghold_adapter
        .set_bytes(
            MIGRATION_VERSION_KEY,
            serde_json::to_string(&migration_version)?.as_bytes(),
        )
        .await?;

    // TODO: delete old chrysalis data records

    Ok(())
}
