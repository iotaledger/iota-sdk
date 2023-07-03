// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

/// The kind of storage used by the manager.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum StorageKind {
    /// RocksDB storage.
    #[cfg(feature = "rocksdb")]
    Rocksdb,
    /// Storage backed by a Map in memory.
    Memory,
    /// Wasm storage.
    #[cfg(target_family = "wasm")]
    Wasm,
}

impl Default for StorageKind {
    fn default() -> Self {
        #[cfg(feature = "rocksdb")]
        return Self::Rocksdb;
        #[cfg(target_family = "wasm")]
        return Self::Wasm;
        #[cfg(not(any(feature = "rocksdb", target_family = "wasm")))]
        Self::Memory
    }
}
