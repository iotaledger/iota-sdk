// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::fmt::Debug;

use serde::{
    ser::{SerializeMap, Serializer},
    Serialize,
};

use crate::types::block::{address::Bech32Address, payload::signed_transaction::TransactionId};

/// The wallet error type.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Account alias must be unique.
    #[error("can't create account: account alias {0} already exists")]
    AccountAliasAlreadyExists(String),
    /// Account not found
    #[error("account {0} not found")]
    AccountNotFound(String),
    /// Address not found in account
    #[error("address {0} not found in account")]
    AddressNotFoundInAccount(Bech32Address),
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
    /// Funds are spread over too many outputs
    #[error("funds are spread over too many outputs {output_count}/{output_count_max}, consolidation required")]
    ConsolidationRequired { output_count: usize, output_count_max: u16 },
    /// Crypto.rs error
    #[error("{0}")]
    Crypto(#[from] crypto::Error),
    /// Custom input error
    #[error("custom input error {0}")]
    CustomInput(String),
    /// Failed to get remainder
    #[error("failed to get remainder address")]
    FailedToGetRemainder,
    /// Insufficient funds to send transaction.
    #[error("insufficient funds {available}/{required} available")]
    InsufficientFunds { available: u64, required: u64 },
    /// Invalid coin type, all accounts need to have the same coin type
    #[error("invalid coin type for new account: {new_coin_type}, existing coin type is: {existing_coin_type}")]
    InvalidCoinType {
        new_coin_type: u32,
        existing_coin_type: u32,
    },
    /// Invalid mnemonic error
    #[error("invalid mnemonic: {0}")]
    InvalidMnemonic(String),
    /// Invalid output kind.
    #[error("invalid output kind: {0}")]
    InvalidOutputKind(String),
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
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    #[error("invalid voting power")]
    InvalidVotingPower,
}

// Serialize type with Display error
impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_map(Some(2))?;
        let mut kind_dbg = format!("{self:?}");
        // Convert first char to lowercase
        if let Some(r) = kind_dbg.get_mut(0..1) {
            r.make_ascii_lowercase();
        }
        // Split by whitespace for struct variants and split by `(` for tuple variants
        // Safe to unwrap because kind_dbg is never an empty string
        let kind = kind_dbg.split([' ', '(']).next().unwrap();
        seq.serialize_entry("type", &kind)?;
        seq.serialize_entry("error", &self.to_string())?;
        seq.end()
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
