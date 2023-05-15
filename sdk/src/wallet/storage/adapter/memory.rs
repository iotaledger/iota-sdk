// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::sync::Arc;
use std::collections::HashMap;

use tokio::sync::RwLock;

use super::StorageAdapter;

/// The storage id.
pub const STORAGE_ID: &str = "Memory";

/// A storage adapter that stores data in memory.
#[derive(Debug, Default)]
pub struct Memory(Arc<RwLock<HashMap<String, String>>>);

#[async_trait::async_trait]
impl StorageAdapter for Memory {
    fn id(&self) -> &'static str {
        STORAGE_ID
    }

    /// Gets the record associated with the given key from the storage.
    async fn get(&self, key: &str) -> crate::wallet::Result<Option<String>> {
        Ok(self.0.read().await.get(key).cloned())
    }

    /// Saves or updates a record on the storage.
    async fn set(&self, key: &str, record: String) -> crate::wallet::Result<()> {
        self.0.write().await.insert(key.to_string(), record);
        Ok(())
    }

    /// Batch writes records to the storage.
    async fn batch_set(&self, records: HashMap<String, String>) -> crate::wallet::Result<()> {
        self.0.write().await.extend(records.into_iter());
        Ok(())
    }

    /// Removes a record from the storage.
    async fn remove(&self, key: &str) -> crate::wallet::Result<()> {
        self.0.write().await.remove(key);
        Ok(())
    }
}
