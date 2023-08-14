// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{
    fmt::Debug,
    path::{Path, PathBuf},
    sync::Arc,
};

use jammdb::{OpenOptions, DB};
use tokio::sync::Mutex;

use super::StorageAdapter;

/// The storage id.
pub const STORAGE_ID: &str = "JammDB";

/// Default storage name
pub const BUCKET_NAME: &str = "Storage";

impl Debug for JammdbStorageAdapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "JammdbStorageAdapter")
    }
}

/// Key value storage adapter.
#[derive(Clone)]
pub struct JammdbStorageAdapter {
    db: Arc<Mutex<DB>>,
}

impl JammdbStorageAdapter {
    /// Initialises the storage adapter.
    pub fn new(path: impl AsRef<Path>) -> crate::wallet::Result<Self> {
        let mut db_path = path.as_ref().to_owned();
        if db_path.is_dir() {
            db_path.push("sdk-wallet.db");
        }
        let db = OpenOptions::new().pagesize(4096).num_pages(32).open(db_path)?;
        // create a default bucket
        let tx = db.tx(true)?;
        let bucket = tx.get_or_create_bucket(BUCKET_NAME)?;
        bucket.put("INITIAL_KEY", "INIT_VALUE")?; // needs some initial value
        tx.commit()?;
        Ok(Self {
            db: Arc::new(Mutex::new(db)),
        })
    }
}

#[async_trait::async_trait]
impl StorageAdapter for JammdbStorageAdapter {
    type Error = crate::wallet::Error;

    async fn get_bytes(&self, key: &str) -> Result<Option<Vec<u8>>, Self::Error> {
        let db = self.db.lock().await;
        let tx = db.tx(false)?;
        let bucket = tx.get_bucket(BUCKET_NAME)?;
        Ok(bucket.get(key).map(|r| r.kv().value().into()))
    }

    async fn set_bytes(&self, key: &str, record: &[u8]) -> Result<(), Self::Error> {
        let db = self.db.lock().await;
        let tx = db.tx(true)?;
        let bucket = tx.get_bucket(BUCKET_NAME)?;
        bucket.put(key, record)?;
        tx.commit()?;
        Ok(())
    }

    async fn delete(&self, key: &str) -> crate::wallet::Result<()> {
        let db = self.db.lock().await;
        let tx = db.tx(true)?;
        let bucket = tx.get_bucket(BUCKET_NAME)?;

        bucket.delete(key)?;
        tx.commit()?;
        Ok(())
    }
}
