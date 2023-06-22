// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{path::Path, sync::Arc};

use rocksdb::{DBCompressionType, Options, DB};
use tokio::sync::Mutex;

use crate::client::storage::StorageAdapter;

/// Key value storage adapter.
#[derive(Debug)]
pub struct RocksdbStorageAdapter {
    db: Arc<Mutex<DB>>,
}

impl RocksdbStorageAdapter {
    /// Initialises the storage adapter.
    pub fn new(path: impl AsRef<Path>) -> crate::wallet::Result<Self> {
        let mut opts = Options::default();
        opts.set_compression_type(DBCompressionType::Lz4);
        opts.create_if_missing(true);
        opts.create_missing_column_families(true);
        let db = DB::open(&opts, path)?;
        Ok(Self {
            db: Arc::new(Mutex::new(db)),
        })
    }
}

#[async_trait::async_trait]
impl StorageAdapter for RocksdbStorageAdapter {
    type Error = crate::wallet::Error;

    async fn get_bytes(&self, key: &str) -> crate::wallet::Result<Option<Vec<u8>>> {
        Ok(self.db.lock().await.get(key)?)
    }

    async fn set_bytes(&self, key: &str, record: &[u8]) -> crate::wallet::Result<()> {
        self.db.lock().await.put(key, record)?;
        Ok(())
    }

    async fn delete(&self, key: &str) -> crate::wallet::Result<()> {
        self.db.lock().await.delete(key)?;
        Ok(())
    }
}
