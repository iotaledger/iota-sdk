// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Error handling in iota-client crate.

use std::fmt::Debug;

use packable::error::UnexpectedEOF;
use serde::{ser::Serializer, Serialize};

use crate::{
    client::api::transaction_builder::TransactionBuilderError,
    types::block::{
        address::AddressError,
        context_input::ContextInputError,
        input::InputError,
        mana::ManaError,
        output::{
            feature::FeatureError, unlock_condition::UnlockConditionError, NativeTokenError, OutputError,
            TokenSchemeError,
        },
        payload::PayloadError,
        semantic::TransactionFailureReason,
        signature::SignatureError,
        unlock::UnlockError,
        BlockError,
    },
    utils::ConversionError,
};

/// Error type of the iota client crate.
#[derive(Debug, thiserror::Error, strum::AsRefStr)]
#[strum(serialize_all = "camelCase")]
#[non_exhaustive]
pub enum ClientError {
    /// Invalid bech32 HRP, should match the one from the used network
    #[error("invalid bech32 hrp for the connected network: {provided}, expected: {expected}")]
    Bech32HrpMismatch {
        /// The bech32 human readable part from the provided address.
        provided: String,
        /// The expected bech32 human readable part.
        expected: String,
    },
    /// Blake2b256 Error
    #[error("{0}")]
    Blake2b256(&'static str),
    /// Block types error
    #[error("{0}")]
    Block(#[from] BlockError),
    /// Address types error
    #[error("{0}")]
    Address(#[from] AddressError),
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
    /// Invalid mnemonic error
    #[error("invalid mnemonic {0}")]
    InvalidMnemonic(String),
    /// JSON error
    #[error("{0}")]
    Json(#[from] serde_json::Error),
    /// Missing required parameters
    #[error("must provide required parameter: {0}")]
    MissingParameter(&'static str),
    /// Error on API request
    #[error("node error: {0}")]
    Node(#[from] crate::client::node_api::error::Error),
    /// Requested output id not found for this type
    #[error("No output found for {0}")]
    NoOutput(String),
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
    /// The transaction could not be accepted
    #[error("transaction ID `{0}` couldn't be accepted")]
    TransactionAcceptance(String),
    #[cfg(not(target_family = "wasm"))]
    /// Tokio task join error
    #[error("{0}")]
    TaskJoin(#[from] tokio::task::JoinError),
    /// Local time doesn't match the network time
    #[error("local time {current_time} doesn't match the tangle time: {tangle_time}")]
    TimeNotSynced {
        /// The local time.
        current_time: u64,
        /// The tangle time.
        tangle_time: u64,
    },
    /// The semantic validation of a transaction failed.
    #[error("the semantic validation of a transaction failed with conflict reason: {} - {0:?}", *.0 as u8)]
    TransactionSemantic(#[from] TransactionFailureReason),
    /// Unpack error
    #[error("{0}")]
    Unpack(#[from] packable::error::UnpackError<BlockError, UnexpectedEOF>),
    /// URL auth error
    #[error("can't set {0} to URL")]
    UrlAuth(&'static str),
    /// URL error
    #[error("{0}")]
    Url(#[from] url::ParseError),
    /// URL validation error
    #[error("{0}")]
    UrlValidation(String),
    /// Transaction builder error.
    #[error("{0}")]
    TransactionBuilder(#[from] TransactionBuilderError),
    /// Missing BIP32 chain to sign with.
    #[error("missing BIP32 chain to sign with")]
    MissingBip32Chain,
    /// Unexpected block body kind.
    #[error("unexpected block body kind: expected {expected}, found {actual}")]
    UnexpectedBlockBodyKind { expected: u8, actual: u8 },
    /// Missing transaction payload.
    #[error("missing transaction payload")]
    MissingTransactionPayload,
    /// Output not unlockable due to deadzone in expiration unlock condition.
    #[error("output not unlockable due to deadzone in expiration unlock condition")]
    ExpirationDeadzone,

    /// Participation error
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    #[error("{0}")]
    Participation(#[from] crate::types::api::plugins::participation::error::Error),

    /// Ledger error
    #[cfg(feature = "ledger_nano")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ledger_nano")))]
    #[error("{0}")]
    Ledger(Box<crate::client::secret::ledger_nano::Error>),

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
    #[error("{0}")]
    Convert(#[from] ConversionError),
}

// Serialize type with Display error
impl Serialize for ClientError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
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

crate::impl_from_error_via!(ClientError via BlockError:
    PayloadError,
    OutputError,
    InputError,
    NativeTokenError,
    ManaError,
    UnlockConditionError,
    FeatureError,
    TokenSchemeError,
    ContextInputError,
    UnlockError,
    SignatureError,
);

#[cfg(feature = "ledger_nano")]
impl From<crate::client::secret::ledger_nano::Error> for ClientError {
    fn from(value: crate::client::secret::ledger_nano::Error) -> Self {
        Self::Ledger(value.into())
    }
}
