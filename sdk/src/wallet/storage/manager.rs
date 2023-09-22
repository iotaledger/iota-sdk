// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use futures::{StreamExt, TryStreamExt};
use zeroize::Zeroizing;

use crate::{
    client::storage::StorageAdapter,
    types::TryFromDto,
    wallet::{
        account::SyncOptions,
        core::{WalletData, WalletDataDto},
        migration::migrate,
        storage::{constants::*, DynStorageAdapter, Storage},
    },
};

/// Storage manager
#[derive(Debug)]
pub(crate) struct StorageManager {
    pub(crate) storage: Storage,
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

        let storage_manager = Self { storage };

        Ok(storage_manager)
    }

    pub(crate) async fn load_wallet_data(&mut self) -> crate::wallet::Result<Option<WalletData>> {
        if let Some(dto) = self.get::<WalletDataDto>(WALLET_INDEXATION_KEY).await? {
            Ok(Some(WalletData::try_from_dto(dto)?))
        } else {
            Ok(None)
        }
    }

    pub(crate) async fn save_wallet_data(&mut self, wallet_data: &WalletData) -> crate::wallet::Result<()> {
        self.set(&format!("{WALLET_INDEXATION_KEY}"), &WalletDataDto::from(wallet_data))
            .await
    }

    // TODO: remove fn?
    pub(crate) async fn remove_wallet_data(&mut self) -> crate::wallet::Result<()> {
        self.delete(&format!("{WALLET_INDEXATION_KEY}")).await
    }

    pub(crate) async fn set_default_sync_options(
        &self,
        account_index: u32,
        sync_options: &SyncOptions,
    ) -> crate::wallet::Result<()> {
        let key = format!("{WALLET_INDEXATION_KEY}-{WALLET_SYNC_OPTIONS}");
        self.set(&key, &sync_options).await
    }

    pub(crate) async fn get_default_sync_options(&self) -> crate::wallet::Result<Option<SyncOptions>> {
        let key = format!("{WALLET_INDEXATION_KEY}-{WALLET_SYNC_OPTIONS}");
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
        assert!(storage_manager.load_wallet_data().await.unwrap().is_empty());

        let account_details = WalletData::mock();

        storage_manager.save_wallet_data(&account_details).await.unwrap();
        let accounts = storage_manager.load_wallet_data().await.unwrap();
        assert_eq!(accounts.len(), 1);
        assert_eq!(accounts[0].alias(), "Alice");

        storage_manager.remove_wallet_data(0).await.unwrap();
        assert!(storage_manager.load_wallet_data().await.unwrap().is_empty());
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
