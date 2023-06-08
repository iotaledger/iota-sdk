// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use futures::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::{
    client::{
        secret::{SecretManager, SecretManagerDto},
        storage::StorageAdapter,
    },
    wallet::{
        account::{AccountDetails, SyncOptions},
        migration::migrate,
        storage::{constants::*, DynStorageAdapter, Storage},
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

/// Storage manager
#[derive(Debug)]
pub(crate) struct StorageManager {
    pub(crate) storage: Storage,
    // account indexes for accounts in the database
    account_indexes: Vec<u32>,
}

impl StorageManager {
    pub(crate) async fn new(
        storage: impl DynStorageAdapter + 'static,
        encryption_key: impl Into<Option<[u8; 32]>> + Send,
    ) -> crate::wallet::Result<Self> {
        let storage = Storage {
            inner: Box::new(storage) as _,
            encryption_key: encryption_key.into(),
        };
        migrate(&storage).await?;

        // Get the db version or set it
        if let Some(db_schema_version) = storage.get::<u8>(DATABASE_SCHEMA_VERSION_KEY).await? {
            if db_schema_version != DATABASE_SCHEMA_VERSION {
                return Err(crate::wallet::Error::Storage(format!(
                    "unsupported database schema version {db_schema_version}"
                )));
            }
        } else {
            storage
                .set(DATABASE_SCHEMA_VERSION_KEY, &DATABASE_SCHEMA_VERSION)
                .await?;
        };

        let account_indexes = storage.get(ACCOUNTS_INDEXATION_KEY).await?.unwrap_or_default();

        let storage_manager = Self {
            storage,
            account_indexes,
        };

        Ok(storage_manager)
    }

    pub async fn get<T: for<'de> Deserialize<'de>>(&self, key: &str) -> crate::wallet::Result<Option<T>> {
        self.storage.get(key).await
    }

    pub async fn save_wallet_data(&self, wallet_builder: &WalletBuilder) -> crate::wallet::Result<()> {
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
                    self.storage.set(SECRET_MANAGER_KEY, &secret_manager_dto).await?;
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
                        let secret_manager = SecretManager::try_from(secret_manager_dto)?;
                        builder.secret_manager = Some(Arc::new(RwLock::new(secret_manager)));
                    }
                }
            }
            Ok(Some(builder))
        } else {
            Ok(None)
        }
    }

    pub async fn get_accounts(&mut self) -> crate::wallet::Result<Vec<AccountDetails>> {
        if let Some(account_indexes) = self.storage.get(ACCOUNTS_INDEXATION_KEY).await? {
            if self.account_indexes.is_empty() {
                self.account_indexes = account_indexes;
            }
        } else {
            return Ok(Vec::new());
        }

        futures::stream::iter(&self.account_indexes)
            .filter_map(|account_index| async {
                let account_index = *account_index;
                let key = format!("{ACCOUNT_INDEXATION_KEY}{account_index}");
                self.get(&key).await.transpose()
            })
            .try_collect::<Vec<_>>()
            .await
    }

    pub async fn save_account(&mut self, account: &AccountDetails) -> crate::wallet::Result<()> {
        // Only add account index if not already present
        if !self.account_indexes.contains(account.index()) {
            self.account_indexes.push(*account.index());
        }

        self.storage.set(ACCOUNTS_INDEXATION_KEY, &self.account_indexes).await?;
        self.storage
            .set(&format!("{ACCOUNT_INDEXATION_KEY}{}", account.index()), account)
            .await
    }

    pub async fn remove_account(&mut self, account_index: u32) -> crate::wallet::Result<()> {
        self.storage
            .delete(&format!("{ACCOUNT_INDEXATION_KEY}{account_index}"))
            .await?;
        self.account_indexes.retain(|a| a != &account_index);
        self.storage.set(ACCOUNTS_INDEXATION_KEY, &self.account_indexes).await
    }

    pub async fn set_default_sync_options(
        &self,
        account_index: u32,
        sync_options: &SyncOptions,
    ) -> crate::wallet::Result<()> {
        let key = format!("{ACCOUNT_INDEXATION_KEY}{account_index}-{ACCOUNT_SYNC_OPTIONS}");
        self.storage.set(&key, &sync_options).await
    }

    pub async fn get_default_sync_options(&self, account_index: u32) -> crate::wallet::Result<Option<SyncOptions>> {
        let key = format!("{ACCOUNT_INDEXATION_KEY}{account_index}-{ACCOUNT_SYNC_OPTIONS}");
        self.storage.get(&key).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wallet::storage::adapter::memory::Memory;

    #[tokio::test]
    async fn get() {
        #[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
        struct Record {
            a: String,
            b: u32,
            c: i64,
        }

        let rec = Record {
            a: "test".to_string(),
            b: 42,
            c: -420,
        };
        let storage = Memory::default();
        storage.set("key", &rec).await.unwrap();

        let storage_manager = StorageManager::new(storage, None).await.unwrap();
        assert_eq!(Some(rec), storage_manager.get::<Record>("key").await.unwrap());
    }

    #[tokio::test]
    async fn save_remove_account() {
        let mut storage_manager = StorageManager::new(Memory::default(), None).await.unwrap();
        assert!(storage_manager.get_accounts().await.unwrap().is_empty());

        let account_details = AccountDetails::mock();

        storage_manager.save_account(&account_details).await.unwrap();
        let accounts = storage_manager.get_accounts().await.unwrap();
        assert_eq!(accounts.len(), 1);
        assert_eq!(accounts[0].alias(), "Alice");

        storage_manager.remove_account(0).await.unwrap();
        assert!(storage_manager.get_accounts().await.unwrap().is_empty());
    }

    #[tokio::test]
    async fn save_get_wallet_data() {
        let storage_manager = StorageManager::new(Memory::default(), None).await.unwrap();
        assert!(storage_manager.get_wallet_data().await.unwrap().is_none());

        let wallet_builder = WalletBuilder::new();
        storage_manager.save_wallet_data(&wallet_builder).await.unwrap();

        assert!(storage_manager.get_wallet_data().await.unwrap().is_some());
    }
}
