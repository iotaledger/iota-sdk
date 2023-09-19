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
            chrysalis::{migrate_from_chrysalis_data, to_chrysalis_key, CHRYSALIS_STORAGE_KEY},
            latest_backup_migration_version, migrate, MigrationData, MIGRATION_VERSION_KEY,
        },
        ClientOptions, Error as WalletError, Wallet,
    },
};

pub(crate) const WALLET_INDEXATION_KEY: &str = "iota-wallet-account-manager";
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
    migrate(stronghold).await?;

    // Get client_options
    let client_options = stronghold.get(CLIENT_OPTIONS_KEY).await?;

    // Get coin_type
    let coin_type_bytes = stronghold.get_bytes(COIN_TYPE_KEY).await?;
    let coin_type = if let Some(coin_type_bytes) = coin_type_bytes {
        let coin_type = u32::from_le_bytes(
            coin_type_bytes
                .try_into()
                .map_err(|_| WalletError::Backup("invalid coin_type"))?,
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

pub(crate) async fn migrate_snapshot_from_chrysalis_to_stardust(
    stronghold_adapter: &StrongholdAdapter,
) -> crate::wallet::Result<Option<HashMap<String, String>>> {
    use crate::client::Error as ClientError;
    log::debug!("migrate_snapshot_from_chrysalis_to_stardust");
    let stronghold = stronghold_adapter.inner().await;
    let stronghold_client = match stronghold.load_client(b"iota-wallet-records") {
        Ok(client) => client,
        // `iota-wallet-records` was only used in chrysalis
        Err(iota_stronghold::ClientError::ClientDataNotPresent) => return Ok(None),
        Err(e) => {
            return Err(WalletError::Client(Box::new(ClientError::Stronghold(e.into()))));
        }
    };

    let stronghold_store = stronghold_client.store();
    let keys = stronghold_store
        .keys()
        .map_err(|e| WalletError::Client(Box::new(ClientError::Stronghold(e.into()))))?;

    let wallet_indexation_key = to_chrysalis_key(b"iota-wallet-account-indexation", true);
    // check if snapshot contains chrysalis data
    if !keys.iter().any(|k| k == &wallet_indexation_key) {
        return Ok(None);
    }

    let mut chrysalis_data: HashMap<Vec<u8>, String> = HashMap::new();
    for key in keys {
        let value = stronghold_store
            .get(&key)
            .map_err(|e| WalletError::Client(Box::new(ClientError::Stronghold(e.into()))))?;

        let value_utf8 =
            String::from_utf8(value.unwrap()).map_err(|_| WalletError::Migration("invalid utf8".into()))?;

        chrysalis_data.insert(key, value_utf8);
    }
    drop(stronghold_store);
    drop(stronghold_client);
    drop(stronghold);

    let (new_accounts, secret_manager_dto) =
        migrate_from_chrysalis_data(&chrysalis_data, Path::new("wallet.stronghold"), true)?;

    // convert to string keys
    let chrysalis_data_with_string_keys = chrysalis_data
        .iter()
        .map(|(k, v)| {
            Ok((
                // the key bytes are a hash in stronghold
                prefix_hex::encode(k),
                v.clone(),
            ))
        })
        .collect::<crate::wallet::Result<HashMap<String, String>>>()?;

    log::debug!(
        "Chrysalis data: {}",
        serde_json::to_string_pretty(&chrysalis_data_with_string_keys)?
    );

    // store chrysalis data in a new key
    stronghold_adapter
        .set(CHRYSALIS_STORAGE_KEY, &chrysalis_data_with_string_keys)
        .await?;

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
    let migration_version = crate::wallet::migration::migrate_4::Migrate::version();
    stronghold_adapter
        .set(MIGRATION_VERSION_KEY, &migration_version)
        .await?;

    // Remove old entries
    let stronghold = stronghold_adapter.inner().await;
    let stronghold_client = stronghold
        .get_client(b"iota-wallet-records")
        .map_err(|e| WalletError::Client(Box::new(ClientError::Stronghold(e.into()))))?;
    let stronghold_store = stronghold_client.store();

    for key in chrysalis_data.keys() {
        stronghold_store
            .delete(key)
            .map_err(|_| WalletError::Migration("couldn't delete old data".into()))?;
    }

    Ok(Some(chrysalis_data_with_string_keys))
}
