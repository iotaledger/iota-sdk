// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use gloo_storage::LocalStorage;

use super::StorageAdapter;

/// The storage id.
pub const STORAGE_ID: &str = "Wasm";

/// Wasm storage adapter using the browser local storage
#[derive(Debug)]
pub struct WasmAdapter(LocalStorage);

impl WasmAdapter {
    /// Initialises the storage adapter.
    pub fn new() -> crate::wallet::Result<Self> {
        Ok(Self(LocalStorage::new()))
    }
}

#[async_trait::async_trait]
impl StorageAdapter for WasmAdapter {
    /// Gets the record associated with the given key from the storage.
    async fn get(&self, key: &str) -> crate::wallet::Result<String> {
        self.0.get(key)
    }

    /// Saves or updates a record on the storage.
    async fn set(&mut self, key: &str, record: String) -> crate::wallet::Result<()> {
        self.0.set(key, record)
    }

    /// Batch writes records to the storage.
    async fn batch_set(&mut self, records: HashMap<String, String>) -> crate::wallet::Result<()> {
        records.into_iter().map(|s| self.set(s.0, s.1));
        Ok(())
    }

    /// Removes a record from the storage.
    async fn remove(&mut self, key: &str) -> crate::wallet::Result<()> {
        self.0.delete(key)
    }
}
