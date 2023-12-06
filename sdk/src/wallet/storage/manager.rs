// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use zeroize::Zeroizing;

use crate::{
    client::{secret::SecretManage, storage::StorageAdapter},
    types::TryFromDto,
    wallet::{
        core::{WalletData, WalletDataDto},
        migration::migrate,
        operations::syncing::SyncOptions,
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

    pub(crate) async fn load_wallet_data<S: SecretManage>(&mut self) -> crate::wallet::Result<Option<WalletData<S>>> {
        if let Some(dto) = self
            .get::<WalletDataDto<S::GenerationOptions, S::SigningOptions>>(WALLET_DATA_KEY)
            .await?
        {
            Ok(Some(WalletData::try_from_dto(dto)?))
        } else {
            Ok(None)
        }
    }

    pub(crate) async fn save_wallet_data<S: SecretManage>(
        &mut self,
        wallet_data: &WalletData<S>,
    ) -> crate::wallet::Result<()> {
        self.set(WALLET_DATA_KEY, &WalletDataDto::from(wallet_data)).await
    }

    pub(crate) async fn set_default_sync_options(&self, sync_options: &SyncOptions) -> crate::wallet::Result<()> {
        let key = format!("{WALLET_DATA_KEY}-{WALLET_SYNC_OPTIONS}");
        self.set(&key, &sync_options).await
    }

    pub(crate) async fn get_default_sync_options(&self) -> crate::wallet::Result<Option<SyncOptions>> {
        let key = format!("{WALLET_DATA_KEY}-{WALLET_SYNC_OPTIONS}");
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
    use pretty_assertions::assert_eq;
    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::{
        client::secret::{mnemonic::MnemonicSecretManager, SecretManager},
        wallet::{storage::adapter::memory::Memory, WalletBuilder},
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
    async fn save_load_wallet_data() {
        let mut storage_manager = StorageManager::new(Memory::default(), None).await.unwrap();
        assert!(
            storage_manager
                .load_wallet_data::<MnemonicSecretManager>()
                .await
                .unwrap()
                .is_none()
        );

        let wallet_data = WalletData::<MnemonicSecretManager>::mock();

        storage_manager.save_wallet_data(&wallet_data).await.unwrap();
        let wallet = storage_manager
            .load_wallet_data::<MnemonicSecretManager>()
            .await
            .unwrap();
        assert!(matches!(wallet, Some(data) if data.alias == Some("Alice".to_string())));
    }

    #[tokio::test]
    async fn save_load_wallet_builder() {
        let storage_manager = StorageManager::new(Memory::default(), None).await.unwrap();
        assert!(
            WalletBuilder::<MnemonicSecretManager>::load(&storage_manager)
                .await
                .unwrap()
                .is_none()
        );

        let wallet_builder = WalletBuilder::<MnemonicSecretManager>::new();
        wallet_builder.save(&storage_manager).await.unwrap();

        assert!(
            WalletBuilder::<MnemonicSecretManager>::load(&storage_manager)
                .await
                .unwrap()
                .is_some()
        );
    }
}
