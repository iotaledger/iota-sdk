// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod ed25519;

use core::convert::Infallible;

use derive_more::From;

pub use self::ed25519::Ed25519Signature;
use crate::types::block::protocol::{WorkScore, WorkScoreParameters};

#[derive(Debug, PartialEq, Eq, strum::Display, derive_more::From)]
#[allow(missing_docs)]
pub enum SignatureError {
    #[strum(to_string = "invalid signature kind: {0}")]
    InvalidSignatureKind(u8),
    SignaturePublicKeyMismatch {
        expected: String,
        actual: String,
    },
    InvalidPublicKey,
    InvalidSignature,
    #[from]
    #[strum(to_string = "{0}")]
    Crypto(crypto::Error),
}

#[cfg(feature = "std")]
impl std::error::Error for SignatureError {}

impl From<Infallible> for SignatureError {
    fn from(error: Infallible) -> Self {
        match error {}
    }
}

/// A `Signature` contains a signature which is used to unlock a transaction input.
///
/// This is defined as part of the Unspent Transaction Output (UTXO) transaction protocol.
///
/// RFC: <https://github.com/luca-moser/protocol-rfcs/blob/signed-tx-payload/text/0000-transaction-payload/0000-transaction-payload.md#signature-unlock-block>
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, packable::Packable, From)]
#[packable(unpack_error = SignatureError)]
#[packable(tag_type = u8, with_error = SignatureError::InvalidSignatureKind)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(untagged))]
pub enum Signature {
    /// An Ed25519 signature.
    #[packable(tag = Ed25519Signature::KIND)]
    Ed25519(Ed25519Signature),
}

impl core::fmt::Debug for Signature {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Ed25519(signature) => signature.fmt(f),
        }
    }
}

impl Signature {
    /// Returns the signature kind of a `Signature`.
    pub fn kind(&self) -> u8 {
        match self {
            Self::Ed25519(_) => Ed25519Signature::KIND,
        }
    }

    crate::def_is_as_opt!(Signature: Ed25519);
}

impl WorkScore for Signature {
    fn work_score(&self, params: WorkScoreParameters) -> u32 {
        match self {
            Self::Ed25519(ed25519) => ed25519.work_score(params),
        }
    }
}

#[cfg(feature = "serde")]
crate::impl_deserialize_untagged!(Signature: Ed25519);
