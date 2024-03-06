// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_sdk::types::block::{
    mana::ManaError, output::OutputError, payload::PayloadError, semantic::TransactionFailureReason,
    signature::SignatureError, BlockError,
};
use packable::error::UnexpectedEOF;
use serde::{Serialize, Serializer};

/// Error type for the bindings core crate.
#[derive(Debug, thiserror::Error, strum::AsRefStr)]
#[strum(serialize_all = "camelCase")]
#[non_exhaustive]
pub enum Error {
    /// Block errors.
    #[error("{0}")]
    Block(#[from] BlockError),
    /// Output errors.
    #[error("{0}")]
    Output(#[from] OutputError),
    /// Payload errors.
    #[error("{0}")]
    Payload(#[from] PayloadError),
    /// Signature errors.
    #[error("{0}")]
    Signature(#[from] SignatureError),
    /// Mana errors.
    #[error("{0}")]
    Mana(#[from] ManaError),
    /// Semantic errors.
    #[error("{0}")]
    TransactionSemantic(#[from] TransactionFailureReason),
    /// Client errors.
    #[error("{0}")]
    Client(#[from] iota_sdk::client::ClientError),
    /// Wallet errors.
    #[error("{0}")]
    Wallet(#[from] iota_sdk::wallet::WalletError),
    /// Prefix hex errors.
    #[error("{0}")]
    PrefixHex(#[from] prefix_hex::Error),
    /// SerdeJson errors.
    #[error("{0}")]
    SerdeJson(#[from] serde_json::error::Error),
    /// Unpack errors.
    #[error("{0}")]
    Unpack(#[from] packable::error::UnpackError<BlockError, UnexpectedEOF>),
}

#[cfg(feature = "stronghold")]
impl From<iota_sdk::client::stronghold::Error> for Error {
    fn from(error: iota_sdk::client::stronghold::Error) -> Self {
        Self::Client(iota_sdk::client::ClientError::Stronghold(error))
    }
}

#[cfg(feature = "mqtt")]
impl From<iota_sdk::client::node_api::mqtt::Error> for Error {
    fn from(error: iota_sdk::client::node_api::mqtt::Error) -> Self {
        Self::Client(iota_sdk::client::ClientError::Mqtt(error))
    }
}

// Serialize type with Display error.
impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct ErrorDto {
            #[serde(rename = "type")]
            kind: String,
            error: serde_json::Value,
        }

        ErrorDto {
            kind: self.as_ref().to_owned(),
            error: match &self {
                // Only Client and wallet have a proper serde impl
                Self::Client(e) => serde_json::to_value(e).map_err(serde::ser::Error::custom)?,
                Self::Wallet(e) => serde_json::to_value(e).map_err(serde::ser::Error::custom)?,
                _ => serde_json::Value::String(self.to_string()),
            },
        }
        .serialize(serializer)
    }
}
