// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use packable::error::UnexpectedEOF;
use serde::{ser::SerializeMap, Serialize, Serializer};

pub use super::{method::AccountMethod, response::Response};

/// Result type of the bindings core crate.
pub type Result<T> = std::result::Result<T, super::error::Error>;

// TODO: SDK Error instead?
/// Error type for the bindings core crate.
#[derive(Debug, thiserror::Error)]
#[allow(clippy::large_enum_variant)]
pub enum Error {
    /// Client error
    #[error("{0}")]
    Client(#[from] iota_sdk::client::Error),
    /// Wallet error
    #[error("{0}")]
    Wallet(#[from] iota_sdk::wallet::Error),
    /// Block dtos error
    #[error("{0}")]
    BlockDto(#[from] iota_sdk::types::block::DtoError),
    /// Error from block crate.
    #[error("{0}")]
    Block(Box<iota_sdk::types::block::Error>),
    /// JSON error
    #[error("{0}")]
    Json(#[from] serde_json::Error),
    /// Prefix hex string convert error
    #[error("{0}")]
    PrefixHex(#[from] prefix_hex::Error),
    /// MQTT error.
    #[cfg(feature = "mqtt")]
    #[cfg_attr(docsrs, doc(cfg(feature = "mqtt")))]
    #[error("{0}")]
    Mqtt(#[from] iota_sdk::client::node_api::mqtt::Error),
    /// Unpack error
    #[error("{0}")]
    Unpack(#[from] packable::error::UnpackError<iota_sdk::types::block::Error, UnexpectedEOF>),
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

impl From<iota_sdk::types::block::Error> for Error {
    fn from(error: iota_sdk::types::block::Error) -> Self {
        Self::Block(Box::new(error))
    }
}

impl From<iota_sdk::client::stronghold::Error> for Error {
    fn from(error: iota_sdk::client::stronghold::Error) -> Self {
        Self::Client(iota_sdk::client::Error::Stronghold(error))
    }
}
