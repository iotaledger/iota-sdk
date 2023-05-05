// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Error handling in iota-client crate.

use std::fmt::Debug;

use packable::error::UnexpectedEOF;
use serde::{
    ser::{SerializeMap, Serializer},
    Serialize,
};

use crate::{
    client::{api::input_selection::Error as InputSelectionError, node_api::indexer::QueryParameter},
    types::block::semantic::ConflictReason,
};

/// Type alias of `Result` in iota-client
pub type Result<T> = std::result::Result<T, Error>;

/// Error type of the iota client crate.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Block dtos error
    #[error("{0}")]
    ApiTypes(#[from] crate::types::api::core::error::Error),
    /// Blake2b256 Error
    #[error("{0}")]
    Blake2b256(&'static str),
    /// Block types error
    #[error("{0}")]
    Block(#[from] crate::types::block::Error),
    /// The wallet account has enough funds, but split on too many outputs
    #[error("the wallet account has enough funds, but split on too many outputs: {0}, max. is 128, consolidate them")]
    ConsolidationRequired(usize),
    /// Crypto.rs error
    #[error("{0}")]
    Crypto(#[from] crypto::Error),
    /// Address not found
    #[error("address: {address} not found in range: {range}")]
    InputAddressNotFound {
        /// The address that was not found.
        address: String,
        /// The range in which the address was not found.
        range: String,
    },
    /// Invalid amount in API response
    #[error("invalid amount in API response: {0}")]
    InvalidAmount(String),
    /// Invalid BIP32 chain data
    #[error("invalid BIP32 chain data")]
    InvalidBIP32ChainData,
    /// Invalid bech32 HRP, should match the one from the used network
    #[error("invalid bech32 hrp for the connected network: {provided}, expected: {expected}")]
    InvalidBech32Hrp {
        /// The bech32 human readable part from the provided address.
        provided: String,
        /// The expected bech32 human readable part.
        expected: String,
    },
    /// Invalid mnemonic error
    #[error("invalid mnemonic {0}")]
    InvalidMnemonic(String),
    /// The transaction essence is too large
    #[error("the transaction essence is too large. Its length is {length}, max length is {max_length}")]
    InvalidRegularTransactionEssenceLength {
        /// The found length.
        length: usize,
        /// The max supported length.
        max_length: usize,
    },
    /// The transaction payload is too large
    #[error("the transaction payload is too large. Its length is {length}, max length is {max_length}")]
    InvalidTransactionPayloadLength {
        /// The found length.
        length: usize,
        /// The max length.
        max_length: usize,
    },
    /// JSON error
    #[error("{0}")]
    Json(#[from] serde_json::Error),
    /// Missing required parameters
    #[error("must provide required parameter: {0}")]
    MissingParameter(&'static str),
    /// Error on API request
    #[error("node error: {0}")]
    Node(#[from] crate::client::node_api::error::Error),
    /// The block doesn't need to be promoted or reattached
    #[error("block ID `{0}` doesn't need to be promoted or reattached")]
    NoNeedPromoteOrReattach(String),
    /// The requested data was not found.
    #[error("the requested data {0} was not found.")]
    NotFound(String),
    /// Requested output id not found for this type
    #[error("No output for {0}")]
    NoOutput(&'static str),
    /// PlaceholderSecretManager can't be used for address generation or signing
    #[error("placeholderSecretManager can't be used for address generation or signing")]
    PlaceholderSecretManager,
    /// Rw lock failed.
    #[error("rw lock failed")]
    PoisonError,
    /// Prefix hex string convert error
    #[error("{0}")]
    PrefixHex(#[from] prefix_hex::Error),
    /// Error on quorum because not enough nodes are available
    #[error("not enough nodes for quorum: {available_nodes} < {minimum_threshold}")]
    QuorumPoolSizeError {
        /// The number of nodes available for quorum.
        available_nodes: usize,
        /// The minimum quorum threshold.
        minimum_threshold: usize,
    },
    /// Error on reaching quorum
    #[error("failed to reach quorum: {quorum_size} < {minimum_threshold}")]
    QuorumThresholdError {
        /// The current quorum size.
        quorum_size: usize,
        /// The minimum quorum threshold.
        minimum_threshold: usize,
    },
    /// Specifically used for `TryInfo` implementations for `SecretManager`.
    #[error("cannot unwrap a SecretManager: type mismatch!")]
    SecretManagerMismatch,
    /// No node available in the healthy node pool
    #[error("no healthy node available")]
    HealthyNodePoolEmpty,
    /// Error when building tagged_data blocks
    #[error("error when building tagged_data block: {0}")]
    TaggedData(String),
    /// The block cannot be included into the Tangle
    #[error("block ID `{0}` couldn't get included into the Tangle")]
    TangleInclusion(String),
    #[cfg(not(target_family = "wasm"))]
    /// Tokio task join error
    #[error("{0}")]
    TaskJoin(#[from] tokio::task::JoinError),
    /// Local time doesn't match the time of the latest milestone timestamp
    #[error(
        "local time {current_time} doesn't match the time of the latest milestone timestamp: {milestone_timestamp}"
    )]
    TimeNotSynced {
        /// The local time.
        current_time: u32,
        /// The timestamp of the latest milestone.
        milestone_timestamp: u32,
    },
    /// The semantic validation of a transaction failed.
    #[error("the semantic validation of a transaction failed with conflict reason: {} - {0:?}", *.0 as u8)]
    TransactionSemantic(ConflictReason),
    /// Unexpected API response error
    #[error("unexpected API response")]
    UnexpectedApiResponse,
    /// An indexer API request contains a query parameter not supported by the endpoint.
    #[error("an indexer API request contains a query parameter not supported by the endpoint: {0}.")]
    UnsupportedQueryParameter(QueryParameter),
    /// Unpack error
    #[error("{0}")]
    Unpack(#[from] packable::error::UnpackError<crate::types::block::Error, UnexpectedEOF>),
    /// URL auth error
    #[error("can't set {0} to URL")]
    UrlAuth(&'static str),
    /// URL error
    #[error("{0}")]
    Url(#[from] url::ParseError),
    /// URL validation error
    #[error("{0}")]
    UrlValidation(String),
    /// Input selection error.
    #[error("{0}")]
    InputSelection(#[from] InputSelectionError),
    /// Missing BIP32 chain to sign with.
    #[error("missing BIP32 chain to sign with")]
    MissingBip32Chain,

    /// Participation error
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    #[error("{0}")]
    Participation(#[from] crate::types::api::plugins::participation::error::Error),

    /// Ledger error
    #[cfg(feature = "ledger_nano")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ledger_nano")))]
    #[error("{0}")]
    Ledger(#[from] crate::client::secret::ledger_nano::Error),

    /// MQTT error
    #[cfg(feature = "mqtt")]
    #[cfg_attr(docsrs, doc(cfg(feature = "mqtt")))]
    #[error("{0}")]
    Mqtt(#[from] crate::client::node_api::mqtt::Error),

    /// Stronghold error
    #[cfg(feature = "stronghold")]
    #[cfg_attr(docsrs, doc(cfg(feature = "stronghold")))]
    #[error("{0}")]
    Stronghold(#[from] crate::client::stronghold::Error),
}

// Serialize type with Display error
impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
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
