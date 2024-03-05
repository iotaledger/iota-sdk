// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::string::String;
use core::convert::Infallible;

#[derive(Debug, PartialEq, Eq, derive_more::Display)]
#[allow(missing_docs)]
pub enum SignatureError {
    #[display(fmt = "invalid signature kind: {_0}")]
    Kind(u8),
    #[display(fmt = "signature public key mismatch: expected {expected} but got {actual}")]
    PublicKeyMismatch { expected: String, actual: String },
    #[display(fmt = "signature does not match the message: {_0}")]
    SignatureMismatch(String),
    #[display(fmt = "invalid public key hex: {_0}")]
    PublicKeyHex(prefix_hex::Error),
    #[display(fmt = "invalid signature hex: {_0}")]
    SignatureHex(prefix_hex::Error),
    #[display(fmt = "invalid public key bytes: {_0}")]
    PublicKeyBytes(crypto::Error),
    #[display(fmt = "invalid signature bytes: {_0}")]
    SignatureBytes(crypto::Error),
}

#[cfg(feature = "std")]
impl std::error::Error for SignatureError {}

impl From<Infallible> for SignatureError {
    fn from(error: Infallible) -> Self {
        match error {}
    }
}
