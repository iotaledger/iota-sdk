// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::str::FromStr;

use derive_more::{AsRef, Deref, Display, From};

use crate::types::block::{
    address::AddressError,
    output::{NftId, OutputId, StorageScore},
};

/// An [`Address`](super::Address) derived from an NFT ID which can be unlocked by unlocking the corresponding NFT.
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, From, AsRef, Deref, Display, packable::Packable)]
#[as_ref(forward)]
pub struct NftAddress(
    /// BLAKE2b-256 hash of the Output ID that created the NFT.
    NftId,
);

impl NftAddress {
    /// The [`Address`](super::Address) kind of an [`NftAddress`].
    pub const KIND: u8 = 16;
    /// The length of an [`NftAddress`].
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

impl StorageScore for NftAddress {}

impl FromStr for NftAddress {
    type Err = AddressError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(NftId::from_str(s)?))
    }
}

impl From<&OutputId> for NftAddress {
    fn from(output_id: &OutputId) -> Self {
        Self(NftId::from(output_id))
    }
}

impl core::fmt::Debug for NftAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "NftAddress({self})")
    }
}

#[cfg(feature = "serde")]
pub(crate) mod dto {
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

    crate::impl_serde_typed_dto!(NftAddress, NftAddressDto, "nft address");
}
