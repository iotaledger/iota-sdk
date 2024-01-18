// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// The default storage path.
pub const DEFAULT_STORAGE_PATH: &str = "./storage";

/// The default RocksDB storage path.
#[cfg(feature = "rocksdb")]
pub(crate) const ROCKSDB_FOLDERNAME: &str = "walletdb";

pub const fn default_storage_path() -> &'static str {
    #[cfg(feature = "rocksdb")]
    return ROCKSDB_FOLDERNAME;
    #[cfg(not(feature = "rocksdb"))]
    DEFAULT_STORAGE_PATH
}

pub(crate) const DATABASE_SCHEMA_VERSION: u8 = 1;
pub(crate) const DATABASE_SCHEMA_VERSION_KEY: &str = "database-schema-version";

pub(crate) const WALLET_DATA_KEY: &str = "wallet-data";
pub(crate) const WALLET_BUILDER_KEY: &str = "wallet-builder";
pub(crate) const WALLET_SYNC_OPTIONS: &str = "wallet-sync-options";

pub(crate) const SECRET_MANAGER_KEY: &str = "secret-manager";

// #[cfg(feature = "participation")]
// pub(crate) const PARTICIPATION_EVENTS: &str = "participation-events";
// #[cfg(feature = "participation")]
// pub(crate) const PARTICIPATION_CACHED_OUTPUTS: &str = "participation-cached-outputs";
