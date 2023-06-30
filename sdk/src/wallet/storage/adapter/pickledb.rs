// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{
    fmt::{Debug, Formatter, Result},
    path::Path,
    sync::Arc,
};

use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use tokio::sync::Mutex;

use crate::client::storage::StorageAdapter;

/// Key value storage adapter.
pub struct PickledbStorageAdapter {
    db: Arc<Mutex<PickleDb>>,
}

impl Debug for PickledbStorageAdapter {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "PickledbStorageAdapter",)
    }
}

impl PickledbStorageAdapter {
    /// Initialises the storage adapter.
    pub fn new(path: impl AsRef<Path>) -> crate::wallet::Result<Self> {
        let db = if path.as_ref().is_file() {
            PickleDb::load(path, PickleDbDumpPolicy::AutoDump, SerializationMethod::Json)?
        } else {
            PickleDb::new(path, PickleDbDumpPolicy::AutoDump, SerializationMethod::Json)
        };
        Ok(Self {
            db: Arc::new(Mutex::new(db)),
        })
    }
}

#[async_trait::async_trait]
impl StorageAdapter for PickledbStorageAdapter {
    type Error = crate::wallet::Error;

    async fn get_bytes(&self, key: &str) -> crate::wallet::Result<Option<Vec<u8>>> {
        let data: Option<String> = self.db.lock().await.get(key);
        Ok(data.map(|d| d.as_bytes().to_vec()))
    }

    async fn set_bytes(&self, key: &str, record: &[u8]) -> crate::wallet::Result<()> {
        // only works without encryption?
        let data = String::from_utf8(record.to_vec()).expect("invalid utf8");
        self.db.lock().await.set(key, &data)?;
        Ok(())
    }

    async fn delete(&self, key: &str) -> crate::wallet::Result<()> {
        self.db.lock().await.rem(key)?;
        Ok(())
    }
}
