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

use async_trait::async_trait;
use crypto::ciphers::chacha;

use self::adapter::StorageAdapter;
use crate::client::storage::StorageAdapter as ClientStorageAdapter;

#[derive(Debug)]
pub struct Storage {
    inner: Box<dyn StorageAdapter>,
    encryption_key: Option<[u8; 32]>,
}

#[async_trait]
impl ClientStorageAdapter for Storage {
    type Error = crate::wallet::Error;

    async fn get_bytes(&self, key: &str) -> Result<Option<Vec<u8>>, Self::Error> {
        match self.inner.as_ref().get_bytes(key).await? {
            Some(record) => {
                if let Some(encryption_key) = &self.encryption_key {
                    return Ok(Some(chacha::aead_decrypt(encryption_key, &record)?));
                }

                Ok(Some(record))
            }
            None => Ok(None),
        }
    }

    async fn set_bytes(&self, key: &str, record: &[u8]) -> Result<(), Self::Error> {
        if let Some(encryption_key) = &self.encryption_key {
            let encrypted_bytes = chacha::aead_encrypt(encryption_key, record)?;
            self.inner.as_ref().set_bytes(key, &encrypted_bytes).await?
        } else {
            self.inner.as_ref().set_bytes(key, record).await?
        }
        Ok(())
    }

    async fn delete(&self, key: &str) -> Result<(), Self::Error> {
        self.inner.as_ref().delete(key).await
    }
}

impl Storage {
    fn id(&self) -> &'static str {
        self.inner.id()
    }
}

impl Drop for Storage {
    fn drop(&mut self) {
        log::debug!("drop Storage");
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::{client::storage::StorageAdapterId, wallet::storage::adapter::memory::Memory};

    #[test]
    fn id() {
        let storage = Storage {
            inner: Box::<Memory>::default(),
            encryption_key: None,
        };
        assert_eq!(storage.id(), Memory::ID);
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
        storage.set("key", &rec).await.unwrap();
        assert_eq!(Some(rec), storage.get::<Record>("key").await.unwrap());

        storage.delete("key").await.unwrap();
        assert_eq!(None, storage.get::<Record>("key").await.unwrap());
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
        storage.set("key", &rec).await.unwrap();

        assert_eq!(Some(rec), storage.get::<Record>("key").await.unwrap());
    }
}
