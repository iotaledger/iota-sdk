// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::str::FromStr;

use derive_more::{AsRef, Deref, From};

use crate::types::block::{output::AnchorId, Error};

/// An anchor address.
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, From, AsRef, Deref, packable::Packable)]
#[as_ref(forward)]
pub struct AnchorAddress(AnchorId);

impl AnchorAddress {
    /// The [`Address`](crate::types::block::address::Address) kind of an [`AnchorAddress`].
    pub const KIND: u8 = 48;
    /// The length of an [`AnchorAddress`].
    pub const LENGTH: usize = AnchorId::LENGTH;

    /// Creates a new [`AnchorAddress`].
    #[inline(always)]
    pub fn new(id: AnchorId) -> Self {
        Self::from(id)
    }

    /// Returns the [`AnchorId`] of an [`AnchorAddress`].
    #[inline(always)]
    pub fn anchor_id(&self) -> &AnchorId {
        &self.0
    }

    /// Consumes an [`AnchorAddress`] and returns its [`AnchorId`].
    #[inline(always)]
    pub fn into_anchor_id(self) -> AnchorId {
        self.0
    }
}

impl FromStr for AnchorAddress {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(AnchorId::from_str(s)?))
    }
}

impl core::fmt::Display for AnchorAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl core::fmt::Debug for AnchorAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "AnchorAddress({self})")
    }
}

#[cfg(feature = "serde")]
mod dto {
    use serde::{Deserialize, Serialize};

    use super::*;

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct AnchorAddressDto {
        #[serde(rename = "type")]
        kind: u8,
        anchor_id: AnchorId,
    }

    impl From<&AnchorAddress> for AnchorAddressDto {
        fn from(value: &AnchorAddress) -> Self {
            Self {
                kind: AnchorAddress::KIND,
                anchor_id: value.0,
            }
        }
    }

    impl From<AnchorAddressDto> for AnchorAddress {
        fn from(value: AnchorAddressDto) -> Self {
            Self(value.anchor_id)
        }
    }

    impl_serde_typed_dto!(AnchorAddress, AnchorAddressDto, "anchor address");
}
