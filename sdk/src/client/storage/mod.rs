// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Database provider interfaces and implementations.

#[cfg(feature = "stronghold")]
#[cfg_attr(docsrs, doc(cfg(feature = "stronghold")))]
mod stronghold;

use async_trait::async_trait;

#[cfg(feature = "stronghold")]
pub use self::stronghold::StrongholdStorageProvider;

/// The interface for database providers.
#[async_trait]
pub trait StorageProvider {
    type Error;

    /// Get a value out of the database.
    async fn get(&self, k: &[u8]) -> Result<Option<Vec<u8>>, Self::Error>;

    /// Insert a value into the database.
    ///
    /// If there exists a record under the same key as `k`, it will be replaced by the new value (`v`) and returned.
    async fn insert(&self, k: &[u8], v: &[u8]) -> Result<Option<Vec<u8>>, Self::Error>;

    /// Delete a value from the database.
    ///
    /// The deleted value is returned.
    async fn delete(&self, k: &[u8]) -> Result<Option<Vec<u8>>, Self::Error>;
}
