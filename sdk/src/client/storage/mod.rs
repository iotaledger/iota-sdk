// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Database provider interfaces and implementations.

#[cfg(feature = "stronghold")]
#[cfg_attr(docsrs, doc(cfg(feature = "stronghold")))]
mod stronghold;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[cfg(feature = "stronghold")]
pub use self::stronghold::StrongholdStorageAdapter;

/// The storage adapter.
#[async_trait]
pub trait StorageAdapter: std::fmt::Debug + Send + Sync {
    type Error;

    /// Gets the record associated with the given key from the storage.
    async fn get<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Result<Option<T>, Self::Error>
    where
        Self::Error: From<serde_json::Error>,
    {
        Ok(self
            .get_bytes(key)
            .await?
            .map(|b| {
                // Safe because serde_json always serializes valid UTF-8
                let s = unsafe { String::from_utf8_unchecked(b) };
                log::trace!("StorageAdapter::get {s:?}");
                serde_json::from_str(&s)
            })
            .transpose()?)
    }

    async fn get_bytes(&self, key: &str) -> Result<Option<Vec<u8>>, Self::Error>;

    /// Saves or updates a record on the storage.
    async fn set<T: Serialize + Send + Sync + ?Sized>(&self, key: &str, record: &T) -> Result<(), Self::Error>
    where
        Self::Error: From<serde_json::Error>,
    {
        self.set_bytes(key, &serde_json::to_vec(record)?).await?;
        Ok(())
    }

    async fn set_bytes(&self, key: &str, record: &[u8]) -> Result<(), Self::Error>;

    /// Removes a record from the storage.
    async fn delete(&self, key: &str) -> Result<(), Self::Error>;
}
