// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use packable::error::UnexpectedEOF;
use serde::{ser::SerializeMap, Serialize, Serializer};

pub use super::{method::AccountMethod, response::Response};

/// Result type of the bindings core crate.
pub type Result<T> = std::result::Result<T, Error>;

/// Error type for the bindings core crate.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Block errors.
    #[error("{0}")]
    Block(#[from] iota_sdk::types::block::Error),
    /// Client errors.
    #[error("{0}")]
    Client(#[from] iota_sdk::client::Error),
    /// Wallet errors.
    #[error("{0}")]
    Wallet(#[from] iota_sdk::wallet::Error),
    /// Prefix hex errors.
    #[error("{0}")]
    PrefixHex(#[from] prefix_hex::Error),
    /// Unpack errors.
    #[error("{0}")]
    Unpack(#[from] packable::error::UnpackError<iota_sdk::types::block::Error, UnexpectedEOF>),
}

impl From<iota_sdk::client::stronghold::Error> for Error {
    fn from(error: iota_sdk::client::stronghold::Error) -> Self {
        Self::Client(iota_sdk::client::Error::Stronghold(error))
    }
}

impl From<iota_sdk::client::node_api::mqtt::Error> for Error {
    fn from(error: iota_sdk::client::node_api::mqtt::Error) -> Self {
        Self::Client(iota_sdk::client::Error::Mqtt(error))
    }
}

// Serialize type with Display error.
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
