// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::StorageAdapter;
use jammdb::{OpenOptions, DB};
use std::{
    fmt::Debug,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::Mutex;

/// The storage id.
pub const STORAGE_ID: &str = "JammDB";

const BUCKET_NAME: &str = "storage";

impl Debug for JammdbStorageAdapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "JammDbStorageAdapter")
    }
}

/// Key value storage adapter.
pub struct JammdbStorageAdapter {
    db: Arc<Mutex<DB>>,
}

impl JammdbStorageAdapter {
    /// Initialises the storage adapter.
    pub fn new(path: impl AsRef<Path>) -> crate::wallet::Result<Self> {
        let mut db_path = PathBuf::from("./sdk-wallet.db");
        let dir_path = path.as_ref().to_string_lossy().to_string();
        let mut temp_path = PathBuf::from(dir_path);
        if path.as_ref().is_dir() {
            temp_path.push(db_path);
        }
        db_path = temp_path;
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
        match bucket.get(key) {
            Some(r) => Ok(Some(r.kv().value().into())),
            None => Ok(None),
        }
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
