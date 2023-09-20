// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod ed25519;

use derive_more::From;

pub use self::ed25519::Ed25519Signature;
use super::protocol::{WorkScore, WorkScoreStructure};
use crate::types::block::Error;

/// A `Signature` contains a signature which is used to unlock a transaction input.
///
/// This is defined as part of the Unspent Transaction Output (UTXO) transaction protocol.
///
/// RFC: <https://github.com/luca-moser/protocol-rfcs/blob/signed-tx-payload/text/0000-transaction-payload/0000-transaction-payload.md#signature-unlock-block>
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, packable::Packable, From)]
#[packable(unpack_error = Error)]
#[packable(tag_type = u8, with_error = Error::InvalidSignatureKind)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(untagged))]
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

    /// Checks whether the signature is an [`Ed25519Signature`].
    pub fn is_ed25519(&self) -> bool {
        matches!(self, Self::Ed25519(_))
    }

    /// Gets the signature as an actual [`Ed25519Signature`].
    /// PANIC: do not call on a non-ed25519 signature.
    pub fn as_ed25519(&self) -> &Ed25519Signature {
        let Self::Ed25519(sig) = self;
        sig
    }
}

impl WorkScore for Signature {
    fn workscore(&self, workscore_structure: WorkScoreStructure) -> u32 {
        match self {
            Self::Ed25519(_) => workscore_structure.signature_ed25519,
        }
    }
}
