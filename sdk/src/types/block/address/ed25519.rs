// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::str::FromStr;

use crypto::signatures::ed25519::PublicKey;
use derive_more::{AsRef, Deref, From};
use packable::Packable;

use super::RestrictedAddress;
use crate::types::block::Error;

/// An Ed25519 address.
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, From, AsRef, Deref, Packable)]
#[as_ref(forward)]
pub struct Ed25519Address([u8; Self::LENGTH]);

impl Ed25519Address {
    /// The [`Address`](crate::types::block::address::Address) kind of an [`Ed25519Address`].
    pub const KIND: u8 = 0;
    /// The length of an [`Ed25519Address`].
    pub const LENGTH: usize = PublicKey::LENGTH;

    /// Creates a new [`Ed25519Address`].
    #[inline(always)]
    pub fn new(address: [u8; Self::LENGTH]) -> Self {
        Self::from(address)
    }
}

impl RestrictedAddress<Ed25519Address> {
    /// The [`Address`](crate::types::block::address::Address) kind of a
    /// [`RestrictedEd25519Address`](Restricted<Ed25519Address>).
    pub const KIND: u8 = 1;
}

impl FromStr for Ed25519Address {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(prefix_hex::decode(s).map_err(Error::Hex)?))
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
    use crate::{types::block::address::restricted::dto::RestrictedAddressDto, utils::serde::prefix_hex_bytes};

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

    impl_serde_typed_dto!(Ed25519Address, Ed25519AddressDto, "ed25519 address");

    impl From<&RestrictedAddress<Ed25519Address>> for RestrictedAddressDto<Ed25519AddressDto> {
        fn from(value: &RestrictedAddress<Ed25519Address>) -> Self {
            Self {
                address: Ed25519AddressDto {
                    kind: RestrictedAddress::<Ed25519Address>::KIND,
                    pub_key_hash: **value.address(),
                },
                allowed_capabilities: value.allowed_capabilities().into_iter().map(|c| **c).collect(),
            }
        }
    }

    impl From<RestrictedAddressDto<Ed25519AddressDto>> for RestrictedAddress<Ed25519Address> {
        fn from(value: RestrictedAddressDto<Ed25519AddressDto>) -> Self {
            let mut res = Self::new(Ed25519Address::from(value.address));
            if let Some(allowed_capabilities) = value.allowed_capabilities.first() {
                res = res.with_allowed_capabilities(*allowed_capabilities);
            }
            res
        }
    }

    impl_serde_typed_dto!(
        RestrictedAddress<Ed25519Address>,
        RestrictedAddressDto<Ed25519AddressDto>,
        "restricted ed25519 address"
    );
}
