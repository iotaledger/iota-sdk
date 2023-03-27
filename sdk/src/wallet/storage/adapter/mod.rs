// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub mod memory;
/// RocksDB storage adapter.
#[cfg(feature = "rocksdb")]
#[cfg_attr(docsrs, doc(cfg(feature = "rocksdb")))]
pub mod rocksdb;

use std::collections::HashMap;

/// The storage adapter.
#[async_trait::async_trait]
pub trait StorageAdapter: std::fmt::Debug {
    /// Gets the storage identifier (used internally on the default storage adapters)
    fn id(&self) -> &'static str {
        "custom-adapter"
    }

    /// Gets the record associated with the given key from the storage.
    async fn get(&self, key: &str) -> crate::wallet::Result<Option<String>>;

    /// Saves or updates a record on the storage.
    async fn set(&mut self, key: &str, record: String) -> crate::wallet::Result<()>;

    /// Batch writes records to the storage.
    async fn batch_set(&mut self, records: HashMap<String, String>) -> crate::wallet::Result<()>;

    /// Removes a record from the storage.
    async fn remove(&mut self, key: &str) -> crate::wallet::Result<()>;
}
