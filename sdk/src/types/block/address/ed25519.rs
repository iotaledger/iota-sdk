// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::str::FromStr;

use crypto::{
    hashes::{blake2b::Blake2b256, Digest},
    signatures::ed25519::PublicKey,
};
use derive_more::{AsRef, Deref, From};
use packable::Packable;

use crate::types::block::{address::AddressError, output::StorageScore};

/// An [`Address`](super::Address) derived from an Ed25519 public key.
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, From, AsRef, Deref, Packable)]
#[as_ref(forward)]
pub struct Ed25519Address(
    /// BLAKE2b-256 hash of the Ed25519 public key.
    [u8; Self::LENGTH],
);

impl Ed25519Address {
    /// The [`Address`](super::Address) kind of an [`Ed25519Address`].
    pub const KIND: u8 = 0;
    /// The length of an [`Ed25519Address`].
    pub const LENGTH: usize = PublicKey::LENGTH;

    /// Creates a new [`Ed25519Address`].
    #[inline(always)]
    pub const fn new(address: [u8; Self::LENGTH]) -> Self {
        Self(address)
    }

    pub(crate) const fn null() -> Self {
        Self([0; Self::LENGTH])
    }

    /// Creates a new [`Ed25519Address`] from the bytes of a [`PublicKey`].
    pub fn from_public_key_bytes(public_key_bytes: [u8; PublicKey::LENGTH]) -> Self {
        Self::new(Blake2b256::digest(public_key_bytes).into())
    }
}

impl StorageScore for Ed25519Address {}

impl FromStr for Ed25519Address {
    type Err = AddressError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(prefix_hex::decode(s).map_err(AddressError::Hex)?))
    }
}

impl core::fmt::Display for Ed25519Address {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", prefix_hex::encode(self.0))
    }
}

impl core::fmt::Debug for Ed25519Address {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Ed25519Address({self})")
    }
}

#[cfg(feature = "serde")]
pub(crate) mod dto {
    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::utils::serde::prefix_hex_bytes;

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Ed25519AddressDto {
        #[serde(rename = "type")]
        kind: u8,
        #[serde(with = "prefix_hex_bytes")]
        pub_key_hash: [u8; Ed25519Address::LENGTH],
    }

    impl From<&Ed25519Address> for Ed25519AddressDto {
        fn from(value: &Ed25519Address) -> Self {
            Self {
                kind: Ed25519Address::KIND,
                pub_key_hash: value.0,
            }
        }
    }

    impl From<Ed25519AddressDto> for Ed25519Address {
        fn from(value: Ed25519AddressDto) -> Self {
            Self(value.pub_key_hash)
        }
    }

    crate::impl_serde_typed_dto!(Ed25519Address, Ed25519AddressDto, "ed25519 address");
}
