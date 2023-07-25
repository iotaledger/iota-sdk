// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::{fmt, ops::Deref};

use crypto::{
    hashes::{blake2b::Blake2b256, Digest},
    signatures::ed25519::{PublicKey, Signature},
};
use packable::{
    error::{UnpackError, UnpackErrorExt},
    packer::Packer,
    unpacker::Unpacker,
    Packable,
};

use crate::types::block::{address::Ed25519Address, Error};

/// An Ed25519 signature.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Ed25519Signature {
    public_key: PublicKey,
    signature: Signature,
}

impl Ed25519Signature {
    /// The signature kind of an [`Ed25519Signature`].
    pub const KIND: u8 = 0;
    /// Length of an ED25519 public key.
    pub const PUBLIC_KEY_LENGTH: usize = PublicKey::LENGTH;
    /// Length of an ED25519 signature.
    pub const SIGNATURE_LENGTH: usize = Signature::LENGTH;

    /// Creates a new [`Ed25519Signature`].
    pub fn new(public_key: PublicKey, signature: Signature) -> Self {
        Self { public_key, signature }
    }

    /// Creates a new [`Ed25519Signature`] from bytes.
    pub fn try_from_bytes(
        public_key: [u8; Self::PUBLIC_KEY_LENGTH],
        signature: [u8; Self::SIGNATURE_LENGTH],
    ) -> Result<Self, Error> {
        Ok(Self::new(
            PublicKey::try_from_bytes(public_key)?,
            Signature::from_bytes(signature),
        ))
    }

    /// Returns the public key of an [`Ed25519Signature`].
    pub fn public_key(&self) -> &PublicKey {
        &self.public_key
    }

    /// Return the actual signature of an [`Ed25519Signature`].
    pub fn signature(&self) -> &Signature {
        &self.signature
    }

    pub fn verify(&self, message: &[u8]) -> bool {
        self.public_key.verify(&self.signature, message)
    }

    /// Verifies the [`Ed25519Signature`] for a message against an [`Ed25519Address`].
    pub fn is_valid(&self, message: &[u8], address: &Ed25519Address) -> Result<(), Error> {
        let signature_address: [u8; Self::PUBLIC_KEY_LENGTH] = Blake2b256::digest(self.public_key).into();

        if address.deref() != &signature_address {
            return Err(Error::SignaturePublicKeyMismatch {
                expected: prefix_hex::encode(address.as_ref()),
                actual: prefix_hex::encode(signature_address),
            });
        }

        if !self.verify(message) {
            return Err(Error::InvalidSignature);
        }

        Ok(())
    }
}

impl fmt::Debug for Ed25519Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[repr(transparent)]
        struct UnquotedStr<'a>(&'a str);

        impl<'a> fmt::Debug for UnquotedStr<'a> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        f.debug_struct("Ed25519Signature")
            .field(
                "public_key",
                &UnquotedStr(&prefix_hex::encode(self.public_key.as_slice())),
            )
            .field(
                "signature",
                &UnquotedStr(&prefix_hex::encode(self.signature.to_bytes())),
            )
            .finish()
    }
}

impl Packable for Ed25519Signature {
    type UnpackError = Error;
    type UnpackVisitor = ();

    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        self.public_key.to_bytes().pack(packer)?;
        self.signature.to_bytes().pack(packer)?;

        Ok(())
    }

    fn unpack<U: Unpacker, const VERIFY: bool>(
        unpacker: &mut U,
        visitor: &Self::UnpackVisitor,
    ) -> Result<Self, UnpackError<Self::UnpackError, U::Error>> {
        let public_key = <[u8; Self::PUBLIC_KEY_LENGTH]>::unpack::<_, VERIFY>(unpacker, visitor).coerce()?;
        let signature = <[u8; Self::SIGNATURE_LENGTH]>::unpack::<_, VERIFY>(unpacker, visitor).coerce()?;
        Self::try_from_bytes(public_key, signature)
            .map_err(UnpackError::Packable)
            .coerce()
    }
}

mod dto {
    use alloc::{format, string::String};

    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::types::block::Error;

    /// Defines an Ed25519 signature.
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Ed25519SignatureDto {
        #[serde(rename = "type")]
        kind: u8,
        public_key: String,
        signature: String,
    }

    impl From<&Ed25519Signature> for Ed25519SignatureDto {
        fn from(value: &Ed25519Signature) -> Self {
            Self {
                kind: Ed25519Signature::KIND,
                public_key: prefix_hex::encode(value.public_key.as_slice()),
                signature: prefix_hex::encode(value.signature.to_bytes()),
            }
        }
    }

    impl TryFrom<Ed25519SignatureDto> for Ed25519Signature {
        type Error = Error;

        fn try_from(value: Ed25519SignatureDto) -> Result<Self, Self::Error> {
            Self::try_from_bytes(
                prefix_hex::decode(&value.public_key).map_err(|_| Error::InvalidField("publicKey"))?,
                prefix_hex::decode(&value.signature).map_err(|_| Error::InvalidField("signature"))?,
            )
        }
    }

    impl<'de> Deserialize<'de> for Ed25519Signature {
        fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            let dto = Ed25519SignatureDto::deserialize(d)?;
            if dto.kind != Self::KIND {
                return Err(serde::de::Error::custom(format!(
                    "invalid signature type: expected {}, found {}",
                    Self::KIND,
                    dto.kind
                )));
            }
            dto.try_into().map_err(serde::de::Error::custom)
        }
    }

    impl Serialize for Ed25519Signature {
        fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            Ed25519SignatureDto::from(self).serialize(s)
        }
    }
}
