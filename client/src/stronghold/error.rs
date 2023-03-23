// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// Stronghold related errors.
#[derive(Debug, thiserror::Error)]
#[allow(clippy::large_enum_variant)]
pub enum Error {
    /// Crypto error.
    #[error("{0}")]
    Crypto(#[from] crypto::Error),
    /// Stronghold client error
    #[error("stronghold client error: {0}")]
    StrongholdClient(#[from] iota_stronghold::ClientError),
    /// Invalid stronghold password.
    #[error("invalid stronghold password")]
    StrongholdInvalidPassword,
    /// No password has been supplied to a Stronghold vault, or it has been cleared
    #[error("no password has been supplied, or the key has been cleared from the memory")]
    StrongholdKeyCleared,
    /// Stronghold memory error
    #[error("stronghold memory error: {0}")]
    StrongholdMemory(#[from] iota_stronghold::MemoryError),
    /// A mnemonic has been already stored into a Stronghold vault
    #[error("a mnemonic has already been stored in the Stronghold vault")]
    StrongholdMnemonicAlreadyStored,
    /// No mnemonic has been stored into the Stronghold vault
    #[error("no mnemonic has been stored into the Stronghold vault")]
    StrongholdMnemonicMissing,
    /// Procedure execution error from Stronghold
    #[error("Stronghold reported a procedure error: {0}")]
    StrongholdProcedure(#[from] iota_stronghold::procedures::ProcedureError),
}
