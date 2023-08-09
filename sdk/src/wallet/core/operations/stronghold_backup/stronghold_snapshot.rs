// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{collections::HashMap, path::Path, sync::atomic::Ordering};

use crate::{
    client::{
        constants::IOTA_COIN_TYPE, secret::SecretManagerConfig, storage::StorageAdapter, stronghold::StrongholdAdapter,
    },
    types::TryFromDto,
    wallet::{
        account::{AccountDetails, AccountDetailsDto},
        migration::{
            chrysalis::{key_to_chrysalis_key, migrate_from_chrysalis_data, CHRYSALIS_STORAGE_KEY},
            latest_backup_migration_version, migrate, MigrationVersion, MIGRATION_VERSION_KEY,
        },
        storage::constants::WALLET_INDEXATION_KEY,
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
        .load_client(b"iota-wallet-records".to_vec())
        .map_err(|e| crate::wallet::Error::Client(Box::new(crate::client::Error::Stronghold(e.into()))))?;
    let stronghold_store = stronghold_client.store();
    let keys = stronghold_store
        .keys()
        .map_err(|e| crate::wallet::Error::Client(Box::new(crate::client::Error::Stronghold(e.into()))))?;

    // check if snapshot contains chrysalis data
    if !keys
        .iter()
        .any(|k| k == &key_to_chrysalis_key("iota-wallet-account-indexation".as_bytes()))
    {
        return Ok(());
    }
    // TODO: are there snapshots with chrysalis AND stardust data? From shimmer claiming for example
    // What to do with them? Following would also move the stardust account data in the chrysalis data

    let mut chrysalis_data: HashMap<Vec<u8>, String> = HashMap::new();
    for key in keys {
        let value = stronghold_store
            .get(&key)
            .map_err(|e| crate::wallet::Error::Client(Box::new(crate::client::Error::Stronghold(e.into()))))?;

        let value_utf8 =
            String::from_utf8(value.unwrap()).map_err(|_| crate::wallet::Error::Migration("invalid utf8".into()))?;

        chrysalis_data.insert(key, value_utf8);
    }
    drop(stronghold_store);
    drop(stronghold_client);
    drop(stronghold);

    let (new_accounts, secret_manager_dto) =
        migrate_from_chrysalis_data(&chrysalis_data, &Path::new("wallet.stronghold"), true)?;

    // convert to string keys
    let chrysalis_data_with_string_keys = chrysalis_data
        .into_iter()
        .map(|(k, v)| {
            Ok((
                // the key bytes are a hash in stronghold
                // TODO: do we want to match against the known keys and replace them so they match what's in the db?
                // Could be complicated since some keys are generated based on data in other values
                prefix_hex::encode(k),
                v,
            ))
        })
        .collect::<crate::wallet::Result<HashMap<String, String>>>()?;

    log::debug!(
        "Chrysalis data: {}",
        serde_json::to_string_pretty(&chrysalis_data_with_string_keys)?
    );

    // TODO: also store in the database?
    // store chrysalis data in a new key
    stronghold_adapter
        .set(CHRYSALIS_STORAGE_KEY, &chrysalis_data_with_string_keys)
        .await?;

    // TODO: do we need to validate the address indexes to be sure that there are no gaps? And if there are, just ignore
    // all above a gap?
    stronghold_adapter
        .set(
            ACCOUNTS_KEY,
            &new_accounts
                .into_iter()
                .map(serde_json::to_value)
                .collect::<Result<Vec<serde_json::Value>, serde_json::Error>>()?,
        )
        .await?;

    if let Some(secret_manager_dto) = secret_manager_dto {
        // This is required for the secret manager to be loaded
        stronghold_adapter
            .set(
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
        .set(MIGRATION_VERSION_KEY, &migration_version)
        .await?;

    // TODO: delete old chrysalis data records

    Ok(())
}
