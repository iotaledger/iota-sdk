// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

/// The kind of storage used by the manager.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[non_exhaustive]
pub enum StorageKind {
    /// RocksDB storage.
    #[cfg(feature = "rocksdb")]
    Rocksdb,
    /// JammDB storage
    #[cfg(feature = "jammdb")]
    Jammdb,
    /// Storage backed by a Map in memory.
    Memory,
    /// Wasm storage.
    #[cfg(target_family = "wasm")]
    Wasm,
}

impl Default for StorageKind {
    fn default() -> Self {
        cfg_if::cfg_if!(
            if #[cfg(feature="rocksdb")]{
                return Self::Rocksdb;
            } else if #[cfg(feature="jammdb")] {
                return Self::Jammdb;
            }
            else if #[cfg(target_family="wasm")]{
                return Self::Wasm;
            }else{
                return Self::Memory;
            }
        );
    }
}
