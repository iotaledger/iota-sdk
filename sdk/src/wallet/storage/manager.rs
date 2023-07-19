// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use futures::{StreamExt, TryStreamExt};
use zeroize::Zeroizing;

use crate::{
    client::storage::StorageAdapter,
    wallet::{
        account::{AccountDetails, AccountDetailsDto, SyncOptions},
        migration::migrate,
        storage::{constants::*, DynStorageAdapter, Storage},
    },
};

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
        encryption_key: impl Into<Option<Zeroizing<[u8; 32]>>> + Send,
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

    pub async fn get_accounts(&mut self) -> crate::wallet::Result<Vec<AccountDetails>> {
        if let Some(account_indexes) = self.get(ACCOUNTS_INDEXATION_KEY).await? {
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
                self.get::<AccountDetailsDto>(&key).await.transpose()
            })
            .map(|res| AccountDetails::try_from_dto_unverified(res?))
            .try_collect::<Vec<_>>()
            .await
    }

    pub async fn save_account(&mut self, account: &AccountDetails) -> crate::wallet::Result<()> {
        // Only add account index if not already present
        if !self.account_indexes.contains(account.index()) {
            self.account_indexes.push(*account.index());
        }

        self.set(ACCOUNTS_INDEXATION_KEY, &self.account_indexes).await?;
        self.set(
            &format!("{ACCOUNT_INDEXATION_KEY}{}", account.index()),
            &AccountDetailsDto::from(account),
        )
        .await
    }

    pub async fn remove_account(&mut self, account_index: u32) -> crate::wallet::Result<()> {
        self.delete(&format!("{ACCOUNT_INDEXATION_KEY}{account_index}")).await?;
        self.account_indexes.retain(|a| a != &account_index);
        self.set(ACCOUNTS_INDEXATION_KEY, &self.account_indexes).await
    }

    pub async fn set_default_sync_options(
        &self,
        account_index: u32,
        sync_options: &SyncOptions,
    ) -> crate::wallet::Result<()> {
        let key = format!("{ACCOUNT_INDEXATION_KEY}{account_index}-{ACCOUNT_SYNC_OPTIONS}");
        self.set(&key, &sync_options).await
    }

    pub async fn get_default_sync_options(&self, account_index: u32) -> crate::wallet::Result<Option<SyncOptions>> {
        let key = format!("{ACCOUNT_INDEXATION_KEY}{account_index}-{ACCOUNT_SYNC_OPTIONS}");
        self.get(&key).await
    }
}

#[async_trait::async_trait]
impl StorageAdapter for StorageManager {
    type Error = crate::wallet::Error;

    async fn get_bytes(&self, key: &str) -> Result<Option<Vec<u8>>, Self::Error> {
        self.storage.get_bytes(key).await
    }

    async fn set_bytes(&self, key: &str, record: &[u8]) -> Result<(), Self::Error> {
        self.storage.set_bytes(key, record).await
    }

    async fn delete(&self, key: &str) -> Result<(), Self::Error> {
        self.storage.delete(key).await
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::{
        client::secret::SecretManager,
        wallet::{core::operations::storage::SaveLoadWallet, storage::adapter::memory::Memory, WalletBuilder},
    };

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
        assert!(
            WalletBuilder::<SecretManager>::load(&storage_manager)
                .await
                .unwrap()
                .is_none()
        );

        let wallet_builder = WalletBuilder::<SecretManager>::new();
        wallet_builder.save(&storage_manager).await.unwrap();

        assert!(
            WalletBuilder::<SecretManager>::load(&storage_manager)
                .await
                .unwrap()
                .is_some()
        );
    }
}
