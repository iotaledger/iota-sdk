// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use zeroize::Zeroizing;

use crate::{
    client::storage::StorageAdapter,
    types::TryFromDto,
    wallet::{
        core::{WalletLedger, WalletLedgerDto},
        migration::migrate,
        operations::syncing::SyncOptions,
        storage::{constants::*, DynStorageAdapter, Storage},
        WalletError,
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
    ) -> Result<Self, WalletError> {
        let storage = Storage {
            inner: Box::new(storage) as _,
            encryption_key: encryption_key.into(),
        };
        migrate(&storage).await?;

        // Get the db version or set it
        if let Some(db_schema_version) = storage.get::<u8>(DATABASE_SCHEMA_VERSION_KEY).await? {
            if db_schema_version != DATABASE_SCHEMA_VERSION {
                return Err(WalletError::Storage(format!(
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

    pub(crate) async fn save_wallet_ledger(&self, wallet_ledger: &WalletLedgerDto) -> Result<(), WalletError> {
        self.set(WALLET_LEDGER_KEY, wallet_ledger).await?;
        Ok(())
    }

    pub(crate) async fn load_wallet_ledger(&self) -> Result<Option<WalletLedger>, WalletError> {
        if let Some(dto) = self.get::<WalletLedgerDto>(WALLET_LEDGER_KEY).await? {
            Ok(Some(WalletLedger::try_from_dto(dto)?))
        } else {
            Ok(None)
        }
    }

    pub(crate) async fn set_default_sync_options(&self, sync_options: &SyncOptions) -> Result<(), WalletError> {
        let key = format!("{WALLET_LEDGER_KEY}-{WALLET_SYNC_OPTIONS}");
        self.set(&key, &sync_options).await
    }

    pub(crate) async fn get_default_sync_options(&self) -> Result<Option<SyncOptions>, WalletError> {
        let key = format!("{WALLET_LEDGER_KEY}-{WALLET_SYNC_OPTIONS}");
        self.get(&key).await
    }
}

#[async_trait::async_trait]
impl StorageAdapter for StorageManager {
    type Error = WalletError;

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
    use crypto::keys::bip44::Bip44;
    use pretty_assertions::assert_eq;
    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::{
        client::{constants::SHIMMER_COIN_TYPE, secret::SecretManager},
        types::block::address::{Bech32Address, Ed25519Address},
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
    async fn save_load_wallet_ledger() {
        let storage_manager = StorageManager::new(Memory::default(), None).await.unwrap();
        assert!(storage_manager.load_wallet_ledger().await.unwrap().is_none());

        let wallet_ledger = WalletLedger::test_instance();

        storage_manager
            .save_wallet_ledger(&WalletLedgerDto::from(&wallet_ledger))
            .await
            .unwrap();

        assert_eq!(storage_manager.load_wallet_ledger().await.unwrap(), Some(wallet_ledger));
    }

    #[tokio::test]
    async fn save_load_wallet_builder() {
        let storage_manager = StorageManager::new(Memory::default(), None).await.unwrap();
        assert!(
            WalletBuilder::<SecretManager>::load(&storage_manager)
                .await
                .unwrap()
                .is_none()
        );

        let wallet_address = Bech32Address::new("rms".parse().unwrap(), Ed25519Address::null());
        let wallet_bip_path = Bip44::new(SHIMMER_COIN_TYPE);
        let wallet_alias = "savings".to_string();

        let wallet_builder = WalletBuilder::<SecretManager>::new()
            .with_address(wallet_address.clone())
            .with_bip_path(wallet_bip_path)
            .with_alias(wallet_alias.clone());

        wallet_builder.save(&storage_manager).await.unwrap();

        let restored_wallet_builder = WalletBuilder::<SecretManager>::load(&storage_manager)
            .await
            .unwrap()
            .unwrap();

        assert_eq!(restored_wallet_builder.address, Some(wallet_address));
        assert_eq!(restored_wallet_builder.bip_path, Some(wallet_bip_path));
        assert_eq!(restored_wallet_builder.alias, Some(wallet_alias));
    }
}
