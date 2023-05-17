// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

use crate::wallet::{
    account::{AccountDetails, SyncOptions},
    migration::migrate_storage,
    storage::{constants::*, Storage, StorageAdapter},
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
pub struct StorageManager {
    pub(crate) storage: Storage,
    // account indexes for accounts in the database
    account_indexes: Vec<u32>,
}

impl StorageManager {
    pub(crate) async fn new(
        storage: impl StorageAdapter + Send + Sync + 'static,
        encryption_key: impl Into<Option<[u8; 32]>> + Send,
    ) -> crate::wallet::Result<Self> {
        let storage = Storage {
            inner: Box::new(storage) as _,
            encryption_key: encryption_key.into(),
        };
        migrate_storage(&storage).await?;

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

        let storage_manager = Self {
            storage,
            account_indexes,
        };

        Ok(storage_manager)
    }

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

    pub async fn get_accounts(&mut self) -> crate::wallet::Result<Vec<AccountDetails>> {
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

    pub async fn save_account(&mut self, account: &AccountDetails) -> crate::wallet::Result<()> {
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
        &self,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        client::secret::SecretManager,
        wallet::{
            storage::adapter::memory::{Memory, STORAGE_ID},
            wallet::operations::storage::SaveLoadWallet,
            WalletBuilder,
        },
    };

    #[tokio::test]
    async fn id() {
        let storage_manager = StorageManager::new(Memory::default(), None).await.unwrap();
        assert_eq!(storage_manager.id(), STORAGE_ID);
        assert!(!storage_manager.is_encrypted());
    }

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
        storage.set("key", serde_json::to_string(&rec).unwrap()).await.unwrap();

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
        assert!(
            WalletBuilder::<SecretManager>::get_data(&storage_manager)
                .await
                .unwrap()
                .is_none()
        );

        let wallet_builder = WalletBuilder::<SecretManager>::new();
        wallet_builder.save_data(&storage_manager).await.unwrap();

        assert!(
            WalletBuilder::<SecretManager>::get_data(&storage_manager)
                .await
                .unwrap()
                .is_some()
        );
    }
}
