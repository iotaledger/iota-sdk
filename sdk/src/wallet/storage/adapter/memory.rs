// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::sync::Arc;
use std::collections::HashMap;

use tokio::sync::RwLock;

use crate::client::storage::StorageAdapter;

/// A storage adapter that stores data in memory.
#[derive(Debug, Default)]
pub struct Memory(Arc<RwLock<HashMap<String, Vec<u8>>>>);

#[async_trait::async_trait]
impl StorageAdapter for Memory {
    type Error = crate::wallet::Error;

    async fn get_bytes(&self, key: &str) -> crate::wallet::Result<Option<Vec<u8>>> {
        Ok(self.0.read().await.get(key).cloned())
    }

    async fn set_bytes(&self, key: &str, record: &[u8]) -> crate::wallet::Result<()> {
        self.0.write().await.insert(key.to_string(), record.to_owned());
        Ok(())
    }

    async fn delete(&self, key: &str) -> crate::wallet::Result<()> {
        self.0.write().await.remove(key);
        Ok(())
    }
}
