// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// Stronghold errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Crypto.rs error
    #[error("{0}")]
    Crypto(#[from] crypto::Error),
    /// Stronghold client error
    #[error("stronghold client error: {0}")]
    Client(#[from] iota_stronghold::ClientError),
    /// Invalid stronghold password.
    #[error("invalid stronghold password")]
    InvalidPassword,
    #[error("invalid json {0}")]
    Json(#[from] serde_json::Error),
    /// No password has been supplied to a Stronghold vault, or it has been cleared
    #[error("no password has been supplied, or the key has been cleared from the memory")]
    KeyCleared,
    /// Stronghold memory error
    #[error("stronghold memory error: {0}")]
    Memory(#[from] iota_stronghold::MemoryError),
    /// A mnemonic has been already stored into a Stronghold vault
    #[error("a mnemonic has already been stored in the Stronghold vault")]
    MnemonicAlreadyStored,
    /// No mnemonic has been stored into the Stronghold vault
    #[error("no mnemonic has been stored into the Stronghold vault")]
    MnemonicMissing,
    /// Procedure execution error from Stronghold
    #[error("stronghold reported a procedure error: {0}")]
    Procedure(#[from] iota_stronghold::procedures::ProcedureError),
    // TODO remove later
    /// Invalid mnemonic error
    #[error("invalid mnemonic {0}")]
    InvalidMnemonic(String),
    /// Unsupported snapshot version
    #[error("unsupported snapshot version, expected {expected}, found {found}, migration required")]
    UnsupportedSnapshotVersion {
        /// Found version
        found: u16,
        /// Expected version
        expected: u16,
    },
    /// Migration error
    #[error("stronghold migration error: {0}")]
    Migration(#[from] iota_stronghold::engine::snapshot::migration::Error),
    /// Invalid rounds error
    #[error("invalid number of hash rounds: {0}")]
    InvalidRounds(u32),
    /// Path already exists
    #[error("path already exists: {0}")]
    PathAlreadyExists(std::path::PathBuf),
    /// Io error
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}
