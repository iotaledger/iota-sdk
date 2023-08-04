// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

/// The kind of storage used by the manager.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[non_exhaustive]
pub enum StorageKind {
    /// RocksDB storage.
    #[cfg(feature = "rocksdb")]
    #[cfg(not(feature = "jammdb"))]
    Rocksdb,
    /// JammDB storage
    #[cfg(feature = "jammdb")]
    #[cfg(not(feature = "rocksdb"))]
    Jammdb,
    /// Storage backed by a Map in memory.
    Memory,
    /// Wasm storage.
    #[cfg(target_family = "wasm")]
    Wasm,
}

impl Default for StorageKind {
    fn default() -> Self {
        #[cfg(feature = "rocksdb")]
        #[cfg(not(feature = "jammdb"))]
        return Self::Rocksdb;
        #[cfg(feature = "jammdb")]
        #[cfg(not(feature = "rocksdb"))]
        return Self::Jammdb;
        #[cfg(target_family = "wasm")]
        return Self::Wasm;
        #[cfg(not(any(feature = "rocksdb", target_family = "wasm", feature = "jammdb")))]
        Self::Memory
    }
}
