// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::fmt::Debug;

use crypto::keys::bip44::Bip44;
use serde::{ser::Serializer, Serialize};

use crate::types::block::{address::Bech32Address, output::DelegationId, payload::signed_transaction::TransactionId};

/// The wallet error type.
#[derive(Debug, thiserror::Error, strum::AsRefStr)]
#[strum(serialize_all = "camelCase")]
#[non_exhaustive]
pub enum Error {
    /// Errors during backup creation or restoring
    #[error("backup failed {0}")]
    Backup(&'static str),
    /// Error from block crate.
    #[error("{0}")]
    Block(Box<crate::types::block::Error>),
    /// Burning or melting failed
    #[error("burning or melting failed: {0}")]
    BurningOrMeltingFailed(String),
    /// Client error.
    #[error("`{0}`")]
    Client(Box<crate::client::Error>),
    /// BIP44 coin type mismatch
    #[error("BIP44 mismatch: {new_bip_path:?}, existing bip path is: {old_bip_path:?}")]
    BipPathMismatch {
        new_bip_path: Option<Bip44>,
        old_bip_path: Option<Bip44>,
    },
    /// Crypto.rs error
    #[error("{0}")]
    Crypto(#[from] crypto::Error),
    /// Custom input error
    #[error("custom input error {0}")]
    CustomInput(String),
    #[error("no delegation output found with id {0}")]
    MissingDelegation(DelegationId),
    /// Insufficient funds to send transaction.
    #[error("address owns insufficient funds: {required} base unit required, but {available} base unit available")]
    InsufficientFunds { available: u64, required: u64 },
    /// Invalid event type.
    #[cfg(feature = "events")]
    #[cfg_attr(docsrs, doc(cfg(feature = "events")))]
    #[error("invalid event type: {0}")]
    InvalidEventType(u8),
    /// Invalid mnemonic error
    #[error("invalid mnemonic: {0}")]
    InvalidMnemonic(String),
    /// Invalid output kind.
    #[error("invalid output kind: {0}")]
    InvalidOutputKind(String),
    /// Invalid Voting Power
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    #[error("invalid voting power")]
    InvalidVotingPower,
    /// IO error. (storage, backup, restore)
    #[error("`{0}`")]
    Io(#[from] std::io::Error),
    /// serde_json error.
    #[error("`{0}`")]
    Json(#[from] serde_json::error::Error),
    /// Error migrating storage or backup
    #[error("migration failed {0}")]
    Migration(String),
    /// Minting failed
    #[error("minting failed {0}")]
    MintingFailed(String),
    /// Missing BIP path.
    #[error("missing BIP path")]
    MissingBipPath,
    /// Missing parameter.
    #[error("missing parameter: {0}")]
    MissingParameter(&'static str),
    /// Nft not found in unspent outputs
    #[error("nft not found in unspent outputs")]
    NftNotFoundInUnspentOutputs,
    /// No outputs available for consolidating
    #[error(
        "nothing to consolidate: available outputs: {available_outputs}, consolidation threshold: {consolidation_threshold}"
    )]
    NoOutputsToConsolidate {
        /// The available outputs for consolidation.
        available_outputs: usize,
        /// The consolidation threshold.
        consolidation_threshold: usize,
    },
    /// Errors not covered by other variants.
    #[error(transparent)]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
    /// Participation error
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    #[error("participation error {0}")]
    Participation(#[from] crate::types::api::plugins::participation::error::Error),
    /// Storage access error.
    #[error("error accessing storage: {0}")]
    Storage(String),
    /// Can't use Wallet API because the storage is encrypted
    #[error("can't perform operation while storage is encrypted; use Wallet::set_storage_password to decrypt storage")]
    StorageIsEncrypted,
    /// Tokio task join error
    #[error("{0}")]
    TaskJoin(#[from] tokio::task::JoinError),
    /// Transaction not found
    #[error("transaction {0} not found")]
    TransactionNotFound(TransactionId),
    // TODO more precise error
    /// Voting error
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    #[error("voting error {0}")]
    Voting(String),
    /// Address not the wallet address
    #[error("address {0} is not the wallet address")]
    WalletAddressMismatch(Bech32Address),
    /// Action requires the wallet to be Ed25519 address based
    #[error("tried to perform an action that requires the wallet to be Ed25519 address based")]
    NonEd25519Address,
    /// Implicit account not found.
    #[error("implicit account not found")]
    ImplicitAccountNotFound,
    /// Account not found.
    #[error("account not found")]
    AccountNotFound,
}

impl Error {
    pub fn other<E: 'static + std::error::Error + Send + Sync>(err: E) -> Self {
        Self::Other(Box::new(err) as _)
    }
}

// Serialize type with Display error
impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct ErrorDto {
            #[serde(rename = "type")]
            kind: String,
            error: String,
        }

        ErrorDto {
            kind: self.as_ref().to_owned(),
            error: self.to_string(),
        }
        .serialize(serializer)
    }
}

impl From<crate::types::block::Error> for Error {
    fn from(error: crate::types::block::Error) -> Self {
        Self::Block(Box::new(error))
    }
}

impl From<crate::client::Error> for Error {
    fn from(error: crate::client::Error) -> Self {
        Self::Client(Box::new(error))
    }
}

impl From<crate::client::api::input_selection::Error> for Error {
    fn from(error: crate::client::api::input_selection::Error) -> Self {
        // Map "same" error so it's easier to handle
        match error {
            crate::client::api::input_selection::Error::InsufficientAmount { found, required } => {
                Self::InsufficientFunds {
                    available: found,
                    required,
                }
            }
            _ => Self::Client(Box::new(crate::client::Error::InputSelection(error))),
        }
    }
}

#[cfg(feature = "stronghold")]
impl From<crate::client::stronghold::Error> for Error {
    fn from(error: crate::client::stronghold::Error) -> Self {
        Self::Client(Box::new(crate::client::Error::Stronghold(error)))
    }
}

#[cfg(feature = "ledger_nano")]
impl From<crate::client::secret::ledger_nano::Error> for Error {
    fn from(error: crate::client::secret::ledger_nano::Error) -> Self {
        Self::Client(Box::new(crate::client::Error::Ledger(error)))
    }
}

#[cfg(feature = "rocksdb")]
impl From<rocksdb::Error> for Error {
    fn from(error: rocksdb::Error) -> Self {
        Self::Storage(error.to_string())
    }
}
