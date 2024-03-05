// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::sync::Arc;
use std::collections::HashMap;

use tokio::sync::RwLock;

use crate::{client::storage::StorageAdapter, wallet::WalletError};

/// A storage adapter that stores data in memory.
#[derive(Debug, Default)]
pub struct Memory(Arc<RwLock<HashMap<String, Vec<u8>>>>);

#[async_trait::async_trait]
impl StorageAdapter for Memory {
    type Error = WalletError;

    async fn get_bytes(&self, key: &str) -> Result<Option<Vec<u8>>, WalletError> {
        Ok(self.0.read().await.get(key).cloned())
    }

    async fn set_bytes(&self, key: &str, record: &[u8]) -> Result<(), WalletError> {
        self.0.write().await.insert(key.to_string(), record.to_owned());
        Ok(())
    }

    async fn delete(&self, key: &str) -> Result<(), WalletError> {
        self.0.write().await.remove(key);
        Ok(())
    }
}
