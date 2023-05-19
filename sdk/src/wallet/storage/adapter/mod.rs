// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;

use crate::client::storage::{StorageAdapter as ClientStorageAdapter, StorageAdapterId};

pub mod memory;
/// RocksDB storage adapter.
#[cfg(feature = "rocksdb")]
#[cfg_attr(docsrs, doc(cfg(feature = "rocksdb")))]
pub mod rocksdb;

#[async_trait]
pub trait StorageAdapter: std::fmt::Debug + Send + Sync {
    /// Gets the storage identifier (used internally on the default storage adapters)
    fn id(&self) -> &'static str;

    /// Gets the record associated with the given key from the storage.
    async fn dyn_get(&self, key: &str) -> crate::wallet::Result<Option<String>>;

    /// Saves or updates a record on the storage.
    async fn dyn_set(&self, key: &str, record: String) -> crate::wallet::Result<()>;

    /// Removes a record from the storage.
    async fn dyn_delete(&self, key: &str) -> crate::wallet::Result<()>;
}

#[async_trait]
impl<T: ClientStorageAdapter + StorageAdapterId> StorageAdapter for T
where
    crate::wallet::Error: From<T::Error>,
    T::Error: From<serde_json::Error>,
{
    fn id(&self) -> &'static str {
        T::ID
    }

    async fn dyn_get(&self, key: &str) -> crate::wallet::Result<Option<String>> {
        Ok(self.get(key).await?)
    }

    async fn dyn_set(&self, key: &str, record: String) -> crate::wallet::Result<()> {
        Ok(self.set(key, &record).await?)
    }

    async fn dyn_delete(&self, key: &str) -> crate::wallet::Result<()> {
        Ok(self.delete(key).await?)
    }
}

#[async_trait]
impl ClientStorageAdapter for &dyn StorageAdapter {
    type Error = crate::wallet::Error;

    async fn get_bytes(&self, key: &str) -> Result<Option<Vec<u8>>, Self::Error> {
        self.get_bytes(key).await
    }

    async fn set_bytes(&self, key: &str, record: &[u8]) -> Result<(), Self::Error> {
        self.set_bytes(key, record).await
    }

    async fn delete(&self, key: &str) -> Result<(), Self::Error> {
        self.delete(key).await
    }
}
