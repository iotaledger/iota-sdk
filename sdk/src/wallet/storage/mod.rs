// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// Storage adapter.
pub mod adapter;
/// Storage constants.
pub mod constants;
/// Storage kind.
mod kind;
/// Storage manager.
mod manager;
/// Storage options.
mod options;
// /// Storage functions related to participation.
// #[cfg(feature = "participation")]
// #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
// mod participation;

use async_trait::async_trait;
use crypto::ciphers::chacha;
use zeroize::Zeroizing;

use self::adapter::DynStorageAdapter;
pub(crate) use self::manager::StorageManager;
pub use self::{kind::StorageKind, options::StorageOptions};
use crate::client::storage::StorageAdapter;

#[derive(Debug)]
pub struct Storage {
    pub(crate) inner: Box<dyn DynStorageAdapter>,
    encryption_key: Option<Zeroizing<[u8; 32]>>,
}

#[async_trait]
impl StorageAdapter for Storage {
    type Error = crate::wallet::Error;

    async fn get_bytes(&self, key: &str) -> Result<Option<Vec<u8>>, Self::Error> {
        match self.inner.as_ref().get_bytes(key).await? {
            Some(record) => {
                if let Some(encryption_key) = &self.encryption_key {
                    return Ok(Some(chacha::aead_decrypt(encryption_key.as_ref(), &record)?));
                }

                Ok(Some(record))
            }
            None => Ok(None),
        }
    }

    async fn set_bytes(&self, key: &str, record: &[u8]) -> Result<(), Self::Error> {
        if let Some(encryption_key) = &self.encryption_key {
            let encrypted_bytes = chacha::aead_encrypt(encryption_key.as_ref(), record)?;
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

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::wallet::storage::adapter::memory::Memory;

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
            encryption_key: Some(Zeroizing::new(encryption_key)),
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
