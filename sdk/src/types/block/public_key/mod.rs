// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod ed25519;

use derive_more::From;

pub use self::ed25519::Ed25519PublicKey;
use crate::types::block::Error;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, From, packable::Packable)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(tag = "type", content = "data")
)]
#[packable(unpack_error = Error)]
#[packable(tag_type = u8, with_error = Error::InvalidSignatureKind)]
pub enum PublicKey {
    /// An Ed25519 public key.
    #[packable(tag = Ed25519PublicKey::KIND)]
    Ed25519(Ed25519PublicKey),
}

impl core::fmt::Debug for PublicKey {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Ed25519(public_key) => public_key.fmt(f),
        }
    }
}

impl PublicKey {
    /// Returns the public key kind of a [`PublicKey`].
    pub fn kind(&self) -> u8 {
        match self {
            Self::Ed25519(_) => Ed25519PublicKey::KIND,
        }
    }

    /// Checks whether the public key is an [`Ed25519PublicKey`].
    pub fn is_ed25519(&self) -> bool {
        matches!(self, Self::Ed25519(_))
    }

    /// Gets the public key as an actual [`Ed25519PublicKey`].
    /// NOTE: Will panic if the public key is not a [`Ed25519PublicKey`].
    pub fn as_ed25519(&self) -> &Ed25519PublicKey {
        let Self::Ed25519(public_key) = self;
        public_key
    }
}

#[allow(missing_docs)]
pub mod dto {
    use serde::{Deserialize, Serialize};

    pub use super::ed25519::dto::Ed25519PublicKeyDto;
    use super::*;
    use crate::types::block::Error;

    /// Describes all the different public key types.
    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, From)]
    #[serde(untagged)]
    pub enum PublicKeyDto {
        Ed25519(Ed25519PublicKeyDto),
    }

    impl From<&PublicKey> for PublicKeyDto {
        fn from(value: &PublicKey) -> Self {
            match value {
                PublicKey::Ed25519(s) => Self::Ed25519(s.into()),
            }
        }
    }

    impl TryFrom<PublicKeyDto> for PublicKey {
        type Error = Error;

        fn try_from(value: PublicKeyDto) -> Result<Self, Self::Error> {
            match value {
                PublicKeyDto::Ed25519(s) => Ok(Self::Ed25519(s.try_into()?)),
            }
        }
    }
}
