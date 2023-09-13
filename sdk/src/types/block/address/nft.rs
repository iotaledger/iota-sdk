// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::str::FromStr;

use derive_more::{AsRef, Deref, From};

use super::Restricted;
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

impl Restricted<NftAddress> {
    /// The [`Address`](crate::types::block::address::Address) kind of a
    /// [`RestrictedNftAddress`](Restricted<NftAddress>).
    pub const KIND: u8 = 17;
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

#[cfg(feature = "serde")]
pub(crate) mod dto {
    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::types::block::address::dto::RestrictedDto;

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

    impl_serde_typed_dto!(NftAddress, NftAddressDto, "nft address");

    impl From<&Restricted<NftAddress>> for RestrictedDto<NftAddressDto> {
        fn from(value: &Restricted<NftAddress>) -> Self {
            Self {
                address: NftAddressDto {
                    kind: Restricted::<NftAddress>::KIND,
                    nft_id: value.address.0,
                },
                allowed_capabilities: value.allowed_capabilities.into_iter().map(|c| c.0).collect(),
            }
        }
    }

    impl From<RestrictedDto<NftAddressDto>> for Restricted<NftAddress> {
        fn from(value: RestrictedDto<NftAddressDto>) -> Self {
            let mut res = Self::new(NftAddress::from(value.address));
            if let Some(allowed_capabilities) = value.allowed_capabilities.first() {
                res = res.with_allowed_capabilities(*allowed_capabilities);
            }
            res
        }
    }

    impl_serde_typed_dto!(
        Restricted<NftAddress>,
        RestrictedDto<NftAddressDto>,
        "restricted nft address"
    );
}
