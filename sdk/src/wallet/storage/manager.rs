// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::{Mutex, RwLock};

use crate::{
    client::secret::{SecretManager, SecretManagerDto},
    wallet::{
        account::{Account, SyncOptions},
        storage::{constants::*, Storage, StorageAdapter},
        WalletBuilder,
    },
};

/// The storage used by the manager.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum ManagerStorage {
    /// RocksDB storage.
    #[cfg(feature = "rocksdb")]
    Rocksdb,
    /// Storage backed by a Map in memory.
    Memory,
    /// Wasm storage.
    #[cfg(target_family = "wasm")]
    Wasm,
}

impl Default for ManagerStorage {
    fn default() -> Self {
        #[cfg(feature = "rocksdb")]
        return Self::Rocksdb;
        #[cfg(target_family = "wasm")]
        return Self::Wasm;
        #[cfg(not(any(feature = "rocksdb", target_family = "wasm")))]
        Self::Memory
    }
}

pub(crate) type StorageManagerHandle = Arc<Mutex<StorageManager>>;

/// Sets the storage adapter.
pub(crate) async fn new_storage_manager(
    encryption_key: Option<[u8; 32]>,
    storage: Box<dyn StorageAdapter + Send + Sync + 'static>,
) -> crate::wallet::Result<StorageManagerHandle> {
    let mut storage = Storage {
        inner: storage,
        encryption_key,
    };
    // Get the db version or set it
    if let Some(db_schema_version) = storage.get::<u8>(DATABASE_SCHEMA_VERSION_KEY).await? {
        if db_schema_version != DATABASE_SCHEMA_VERSION {
            return Err(crate::wallet::Error::Storage(format!(
                "unsupported database schema version {db_schema_version}"
            )));
        }
    } else {
        storage
            .set(DATABASE_SCHEMA_VERSION_KEY, DATABASE_SCHEMA_VERSION)
            .await?;
    };

    let account_indexes = storage.get(ACCOUNTS_INDEXATION_KEY).await?.unwrap_or_default();

    let storage_manager = StorageManager {
        storage,
        account_indexes,
    };

    Ok(Arc::new(Mutex::new(storage_manager)))
}

/// Storage manager
#[derive(Debug)]
pub struct StorageManager {
    pub(crate) storage: Storage,
    // account indexes for accounts in the database
    account_indexes: Vec<u32>,
}

impl StorageManager {
    pub fn id(&self) -> &'static str {
        self.storage.id()
    }

    #[cfg(test)]
    pub fn is_encrypted(&self) -> bool {
        self.storage.encryption_key.is_some()
    }

    pub async fn get<T: for<'de> Deserialize<'de>>(&self, key: &str) -> crate::wallet::Result<Option<T>> {
        self.storage.get(key).await
    }

    pub async fn save_wallet_data(&mut self, wallet_builder: &WalletBuilder) -> crate::wallet::Result<()> {
        log::debug!("save_wallet_data");
        self.storage.set(WALLET_INDEXATION_KEY, wallet_builder).await?;

        if let Some(secret_manager) = &wallet_builder.secret_manager {
            let secret_manager = secret_manager.read().await;
            let secret_manager_dto = SecretManagerDto::from(&*secret_manager);
            // Only store secret_managers that aren't SecretManagerDto::Mnemonic, because there the Seed can't be
            // serialized, so we can't create the SecretManager again
            match secret_manager_dto {
                SecretManagerDto::Mnemonic(_) => {}
                _ => {
                    self.storage.set(SECRET_MANAGER_KEY, secret_manager_dto).await?;
                }
            }
        }
        Ok(())
    }

    pub async fn get_wallet_data(&self) -> crate::wallet::Result<Option<WalletBuilder>> {
        log::debug!("get_wallet_data");
        if let Some(mut builder) = self.storage.get::<WalletBuilder>(WALLET_INDEXATION_KEY).await? {
            log::debug!("get_wallet_data {builder:?}");

            if let Some(secret_manager_dto) = self.storage.get::<SecretManagerDto>(SECRET_MANAGER_KEY).await? {
                log::debug!("get_secret_manager {secret_manager_dto:?}");

                // Only secret_managers that aren't SecretManagerDto::Mnemonic can be restored, because there the Seed
                // can't be serialized, so we can't create the SecretManager again
                match secret_manager_dto {
                    SecretManagerDto::Mnemonic(_) => {}
                    _ => {
                        let secret_manager = SecretManager::try_from(&secret_manager_dto)?;
                        builder.secret_manager = Some(Arc::new(RwLock::new(secret_manager)));
                    }
                }
            }
            Ok(Some(builder))
        } else {
            Ok(None)
        }
    }

    pub async fn get_accounts(&mut self) -> crate::wallet::Result<Vec<Account>> {
        if let Some(account_indexes) = self.storage.get(ACCOUNTS_INDEXATION_KEY).await? {
            if self.account_indexes.is_empty() {
                self.account_indexes = account_indexes;
            }
        } else {
            return Ok(Vec::new());
        }

        let mut accounts = Vec::new();
        for account_index in self.account_indexes.clone() {
            // PANIC: we assume that ACCOUNTS_INDEXATION_KEY and the different indexes are set together and
            // ACCOUNTS_INDEXATION_KEY has already been checked.
            accounts.push(
                self.get(&format!("{ACCOUNT_INDEXATION_KEY}{account_index}"))
                    .await?
                    .unwrap(),
            );
        }

        Ok(accounts)
    }

    pub async fn save_account(&mut self, account: &Account) -> crate::wallet::Result<()> {
        // Only add account index if not already present
        if !self.account_indexes.contains(account.index()) {
            self.account_indexes.push(*account.index());
        }

        self.storage
            .set(ACCOUNTS_INDEXATION_KEY, self.account_indexes.clone())
            .await?;
        self.storage
            .set(&format!("{ACCOUNT_INDEXATION_KEY}{}", account.index()), account)
            .await
    }

    pub async fn remove_account(&mut self, account_index: u32) -> crate::wallet::Result<()> {
        self.storage
            .remove(&format!("{ACCOUNT_INDEXATION_KEY}{account_index}"))
            .await?;
        self.account_indexes.retain(|a| a != &account_index);
        self.storage
            .set(ACCOUNTS_INDEXATION_KEY, self.account_indexes.clone())
            .await
    }

    pub async fn set_default_sync_options(
        &mut self,
        account_index: u32,
        sync_options: &SyncOptions,
    ) -> crate::wallet::Result<()> {
        let key = format!("{ACCOUNT_INDEXATION_KEY}{account_index}-{ACCOUNT_SYNC_OPTIONS}");
        self.storage.set(&key, sync_options.clone()).await
    }

    pub async fn get_default_sync_options(&self, account_index: u32) -> crate::wallet::Result<Option<SyncOptions>> {
        let key = format!("{ACCOUNT_INDEXATION_KEY}{account_index}-{ACCOUNT_SYNC_OPTIONS}");
        self.storage.get(&key).await
    }
}
