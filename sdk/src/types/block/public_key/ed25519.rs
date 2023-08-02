// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crypto::signatures::ed25519::PublicKey;
use derive_more::{AsRef, Deref, From};
use packable::{
    error::{UnpackError, UnpackErrorExt},
    packer::Packer,
    unpacker::Unpacker,
    Packable,
};

use crate::types::block::Error;

/// An Ed25519 public key.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Deref, AsRef, From)]
#[as_ref(forward)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Ed25519PublicKey(PublicKey);

impl Ed25519PublicKey {
    /// The public key kind of an [`Ed25519PublicKey`].
    pub const KIND: u8 = 0;
    /// Length of an ED25519 public key.
    pub const PUBLIC_KEY_LENGTH: usize = PublicKey::LENGTH;

    /// Creates a new [`Ed25519PublicKey`] from bytes.
    pub fn try_from_bytes(bytes: [u8; Self::PUBLIC_KEY_LENGTH]) -> Result<Self, Error> {
        Ok(Self(PublicKey::try_from_bytes(bytes)?))
    }
}

impl core::fmt::Debug for Ed25519PublicKey {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", prefix_hex::encode(self.0.as_slice()))
    }
}

impl Packable for Ed25519PublicKey {
    type UnpackError = Error;
    type UnpackVisitor = ();

    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        packer.pack_bytes(self.0.as_slice())?;
        Ok(())
    }

    fn unpack<U: Unpacker, const VERIFY: bool>(
        unpacker: &mut U,
        visitor: &Self::UnpackVisitor,
    ) -> Result<Self, UnpackError<Self::UnpackError, U::Error>> {
        Self::try_from_bytes(<[u8; Self::PUBLIC_KEY_LENGTH]>::unpack::<_, VERIFY>(unpacker, visitor).coerce()?)
            .map_err(UnpackError::Packable)
    }
}

pub(crate) mod dto {
    use alloc::string::String;

    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::types::block::Error;

    /// Defines an Ed25519 public key.
    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Ed25519PublicKeyDto {
        #[serde(rename = "type")]
        pub kind: u8,
        pub public_key: String,
    }

    impl From<&Ed25519PublicKey> for Ed25519PublicKeyDto {
        fn from(value: &Ed25519PublicKey) -> Self {
            Self {
                kind: Ed25519PublicKey::KIND,
                public_key: prefix_hex::encode(value.0.as_slice()),
            }
        }
    }

    impl TryFrom<Ed25519PublicKeyDto> for Ed25519PublicKey {
        type Error = Error;

        fn try_from(value: Ed25519PublicKeyDto) -> Result<Self, Self::Error> {
            Self::try_from_bytes(prefix_hex::decode(value.public_key).map_err(|_| Error::InvalidField("publicKey"))?)
        }
    }
}
