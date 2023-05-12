// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// Storage adapter.
pub mod adapter;
/// Storage constants.
pub mod constants;
/// Storage manager.
pub mod manager;
/// Storage functions related to participation.
#[cfg(feature = "participation")]
#[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
mod participation;

use std::collections::HashMap;

use crypto::ciphers::chacha;
use serde::{Deserialize, Serialize};

use self::adapter::StorageAdapter;

#[derive(Debug)]
pub struct Storage {
    inner: Box<dyn StorageAdapter + Sync + Send>,
    encryption_key: Option<[u8; 32]>,
}

impl Storage {
    fn id(&self) -> &'static str {
        self.inner.id()
    }

    pub(crate) async fn get<T: for<'de> Deserialize<'de>>(&self, key: &str) -> crate::wallet::Result<Option<T>> {
        match self.inner.get(key).await? {
            Some(record) => {
                if let Some(encryption_key) = &self.encryption_key {
                    if let Ok(encrypted_bytes) = serde_json::from_str::<Vec<u8>>(&record) {
                        return Ok(Some(serde_json::from_str(&String::from_utf8_lossy(
                            &chacha::aead_decrypt(encryption_key, &encrypted_bytes)?,
                        ))?));
                    }
                }

                Ok(Some(serde_json::from_str(&record)?))
            }
            None => Ok(None),
        }
    }

    pub(crate) async fn set<T: Serialize + Send>(&self, key: &str, record: T) -> crate::wallet::Result<()> {
        let record = serde_json::to_string(&record)?;
        self.inner
            .set(
                key,
                if let Some(encryption_key) = &self.encryption_key {
                    let encrypted_bytes = chacha::aead_encrypt(encryption_key, record.as_bytes())?;
                    serde_json::to_string(&encrypted_bytes)?
                } else {
                    record
                },
            )
            .await
    }

    #[allow(dead_code)]
    async fn batch_set<T: Serialize + Send>(&self, records: HashMap<String, T>) -> crate::wallet::Result<()> {
        self.inner
            .batch_set(if let Some(key) = &self.encryption_key {
                let mut encrypted_records = HashMap::with_capacity(records.len());
                for (id, record) in records {
                    let encrypted_bytes = chacha::aead_encrypt(key, serde_json::to_string(&record)?.as_bytes())?;
                    encrypted_records.insert(id, serde_json::to_string(&encrypted_bytes)?);
                }
                encrypted_records
            } else {
                let mut plain_records = HashMap::with_capacity(records.len());
                for (id, record) in records {
                    plain_records.insert(id, serde_json::to_string(&record)?);
                }
                plain_records
            })
            .await
    }

    async fn remove(&self, key: &str) -> crate::wallet::Result<()> {
        self.inner.remove(key).await
    }
}

impl Drop for Storage {
    fn drop(&mut self) {
        log::debug!("drop Storage");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wallet::storage::adapter::memory::{Memory, STORAGE_ID};

    #[test]
    fn id() {
        let storage = Storage {
            inner: Box::<Memory>::default(),
            encryption_key: None,
        };
        assert_eq!(storage.id(), STORAGE_ID);
    }

    #[tokio::test]
    async fn get_set_remove() {
        #[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
        struct Record {
            a: String,
            b: u32,
            c: i64,
        }

        let storage = Storage {
            inner: Box::<Memory>::default(),
            encryption_key: None,
        };

        let rec = Record {
            a: "test".to_string(),
            b: 42,
            c: -420,
        };
        storage.set("key", rec.clone()).await.unwrap();
        assert_eq!(Some(rec), storage.get::<Record>("key").await.unwrap());

        storage.remove("key").await.unwrap();
        assert_eq!(None, storage.get::<Record>("key").await.unwrap());
    }

    #[cfg(feature = "rand")]
    #[tokio::test]
    async fn batch_set() {
        #[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
        struct Record {
            a: String,
            b: u32,
            c: i64,
        }

        let storage = Storage {
            inner: Box::<Memory>::default(),
            encryption_key: None,
        };

        let records = std::iter::repeat_with(|| Record {
            a: "test".to_string(),
            b: rand::random(),
            c: rand::random(),
        })
        .enumerate()
        .map(|(key, record)| (key.to_string(), record))
        .take(10)
        .collect::<HashMap<_, _>>();

        storage.batch_set(records.clone()).await.unwrap();

        // we also check an index that doesn't exist
        for index in 0..11 {
            assert_eq!(
                records.get(&index.to_string()),
                storage.get::<Record>(&index.to_string()).await.unwrap().as_ref()
            );
        }
    }

    #[cfg(feature = "rand")]
    #[tokio::test]
    async fn get_set_encrypted() {
        #[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
        struct Record {
            a: String,
            b: u32,
            c: i64,
        }

        let encryption_key = crate::types::block::rand::bytes::rand_bytes_array::<32>();

        let storage = Storage {
            inner: Box::<Memory>::default(),
            encryption_key: Some(encryption_key),
        };

        let rec = Record {
            a: "test".to_string(),
            b: 42,
            c: -420,
        };
        storage.set("key", rec.clone()).await.unwrap();

        assert_eq!(Some(rec), storage.get::<Record>("key").await.unwrap());
    }
}
