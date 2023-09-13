// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::str::FromStr;

use crypto::signatures::ed25519::PublicKey;
use derive_more::{AsRef, Deref, Display, From, FromStr};
use packable::Packable;

use super::Restricted;
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

impl Restricted<Ed25519Address> {
    /// The [`Address`](crate::types::block::address::Address) kind of a
    /// [`RestrictedEd25519Address`](Restricted<Ed25519Address>).
    pub const KIND: u8 = 1;
}

/// An implicit account creation address that can be used to transition an account.
#[derive(Copy, Clone, Debug, Display, Eq, PartialEq, Ord, PartialOrd, Hash, FromStr, AsRef, Deref, From, Packable)]
#[as_ref(forward)]
pub struct ImplicitAccountCreationAddress(Ed25519Address);
impl ImplicitAccountCreationAddress {
    /// The [`Address`](crate::types::block::address::Address) kind of an [`ImplicitAccountCreationAddress`].
    pub const KIND: u8 = 24;
    /// The length of an [`ImplicitAccountCreationAddress`].
    pub const LENGTH: usize = Ed25519Address::LENGTH;

    /// Creates a new [`ImplicitAccountCreationAddress`].
    #[inline(always)]
    pub fn new(address: [u8; Self::LENGTH]) -> Self {
        Self(Ed25519Address::new(address))
    }
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
    use crate::{types::block::address::dto::RestrictedDto, utils::serde::prefix_hex_bytes};

    /// Describes an Ed25519 address.
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

    impl From<&Restricted<Ed25519Address>> for RestrictedDto<Ed25519AddressDto> {
        fn from(value: &Restricted<Ed25519Address>) -> Self {
            Self {
                address: Ed25519AddressDto {
                    kind: Restricted::<Ed25519Address>::KIND,
                    pub_key_hash: value.address.0,
                },
                allowed_capabilities: value.allowed_capabilities.into_iter().map(|c| c.0).collect(),
            }
        }
    }

    impl From<RestrictedDto<Ed25519AddressDto>> for Restricted<Ed25519Address> {
        fn from(value: RestrictedDto<Ed25519AddressDto>) -> Self {
            let mut res = Self::new(Ed25519Address::from(value.address));
            if let Some(allowed_capabilities) = value.allowed_capabilities.first() {
                res = res.with_allowed_capabilities(*allowed_capabilities);
            }
            res
        }
    }

    impl_serde_typed_dto!(
        Restricted<Ed25519Address>,
        RestrictedDto<Ed25519AddressDto>,
        "restricted ed25519 address"
    );

    /// Describes an Implicit Account Creation address.
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct ImplicitAccountCreationAddressDto {
        #[serde(rename = "type")]
        kind: u8,
        #[serde(with = "prefix_hex_bytes")]
        pub_key_hash: [u8; Ed25519Address::LENGTH],
    }

    impl From<&ImplicitAccountCreationAddress> for ImplicitAccountCreationAddressDto {
        fn from(value: &ImplicitAccountCreationAddress) -> Self {
            Self {
                kind: ImplicitAccountCreationAddress::KIND,
                pub_key_hash: value.0.0,
            }
        }
    }

    impl From<ImplicitAccountCreationAddressDto> for ImplicitAccountCreationAddress {
        fn from(value: ImplicitAccountCreationAddressDto) -> Self {
            Self::new(value.pub_key_hash)
        }
    }

    impl_serde_typed_dto!(
        ImplicitAccountCreationAddress,
        ImplicitAccountCreationAddressDto,
        "implicit account creation address"
    );
}
