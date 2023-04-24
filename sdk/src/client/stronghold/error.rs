// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// Error type of the stronghold module.
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
    #[error("Stronghold reported a procedure error: {0}")]
    Procedure(#[from] iota_stronghold::procedures::ProcedureError),
    // TODO remove later
    /// Invalid mnemonic error
    #[error("invalid mnemonic {0}")]
    InvalidMnemonic(String),
}
