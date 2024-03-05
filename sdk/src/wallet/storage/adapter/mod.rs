// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub mod memory;
/// RocksDB storage adapter.
#[cfg(feature = "rocksdb")]
#[cfg_attr(docsrs, doc(cfg(feature = "rocksdb")))]
pub mod rocksdb;

use async_trait::async_trait;

use crate::{client::storage::StorageAdapter, wallet::WalletError};

#[async_trait]
pub(crate) trait DynStorageAdapter: std::fmt::Debug + Send + Sync {
    async fn dyn_get_bytes(&self, key: &str) -> Result<Option<Vec<u8>>, WalletError>;

    async fn dyn_set_bytes(&self, key: &str, record: &[u8]) -> Result<(), WalletError>;

    /// Removes a record from the storage.
    async fn dyn_delete(&self, key: &str) -> Result<(), WalletError>;
}

#[async_trait]
impl<T: StorageAdapter> DynStorageAdapter for T
where
    WalletError: From<T::Error>,
{
    async fn dyn_get_bytes(&self, key: &str) -> Result<Option<Vec<u8>>, WalletError> {
        Ok(self.get_bytes(key).await?)
    }

    async fn dyn_set_bytes(&self, key: &str, record: &[u8]) -> Result<(), WalletError> {
        Ok(self.set_bytes(key, record).await?)
    }

    async fn dyn_delete(&self, key: &str) -> Result<(), WalletError> {
        Ok(self.delete(key).await?)
    }
}

#[async_trait]
impl StorageAdapter for dyn DynStorageAdapter {
    type Error = WalletError;

    async fn get_bytes(&self, key: &str) -> Result<Option<Vec<u8>>, Self::Error> {
        self.dyn_get_bytes(key).await
    }

    async fn set_bytes(&self, key: &str, record: &[u8]) -> Result<(), Self::Error> {
        self.dyn_set_bytes(key, record).await
    }

    async fn delete(&self, key: &str) -> Result<(), Self::Error> {
        self.dyn_delete(key).await
    }
}
