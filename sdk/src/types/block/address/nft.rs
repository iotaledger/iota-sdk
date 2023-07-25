// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::str::FromStr;

use derive_more::{AsRef, Deref, From};

use crate::types::block::{output::NftId, Error};

/// An NFT address.
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, From, AsRef, Deref, packable::Packable)]
#[as_ref(forward)]
pub struct NftAddress(NftId);

impl NftAddress {
    /// The [`Address`](crate::types::block::address::Address) kind of an NFT address.
    pub const KIND: u8 = 16;
    /// The length of a [`NftAddress`].
    pub const LENGTH: usize = NftId::LENGTH;

    /// Creates a new [`NftAddress`].
    #[inline(always)]
    pub fn new(id: NftId) -> Self {
        Self::from(id)
    }

    /// Returns the [`NftId`] of an [`NftAddress`].
    #[inline(always)]
    pub fn nft_id(&self) -> &NftId {
        &self.0
    }

    /// Consumes an [`NftAddress`] and returns its [`NftId`].
    #[inline(always)]
    pub fn into_nft_id(self) -> NftId {
        self.0
    }
}

impl FromStr for NftAddress {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(NftId::from_str(s)?))
    }
}

impl core::fmt::Display for NftAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl core::fmt::Debug for NftAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "NftAddress({self})")
    }
}

mod dto {
    use alloc::format;

    use serde::{Deserialize, Serialize};

    use super::*;

    /// Describes an NFT address.
    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct NftAddressDto {
        #[serde(rename = "type")]
        kind: u8,
        nft_id: NftId,
    }

    impl From<&NftAddress> for NftAddressDto {
        fn from(value: &NftAddress) -> Self {
            Self {
                kind: NftAddress::KIND,
                nft_id: value.0,
            }
        }
    }

    impl From<NftAddressDto> for NftAddress {
        fn from(value: NftAddressDto) -> Self {
            Self(value.nft_id)
        }
    }

    impl<'de> Deserialize<'de> for NftAddress {
        fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            let dto = NftAddressDto::deserialize(d)?;
            if dto.kind != Self::KIND {
                return Err(serde::de::Error::custom(format!(
                    "invalid nft address type: expected {}, found {}",
                    Self::KIND,
                    dto.kind
                )));
            }
            Ok(dto.into())
        }
    }

    impl Serialize for NftAddress {
        fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            NftAddressDto::from(self).serialize(s)
        }
    }
}
