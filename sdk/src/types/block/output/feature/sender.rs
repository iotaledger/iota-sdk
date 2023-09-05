// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use derive_more::From;

use crate::types::block::{
    address::Address,
    output::{rent::RentBuilder, Rent},
};

/// Identifies the validated sender of an output.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, From, packable::Packable)]
pub struct SenderFeature(pub(crate) Address);

impl SenderFeature {
    /// The [`Feature`](crate::types::block::output::Feature) kind of a [`SenderFeature`].
    pub const KIND: u8 = 0;

    /// Creates a new [`SenderFeature`].
    #[inline(always)]
    pub fn new(address: impl Into<Address>) -> Self {
        Self(address.into())
    }

    /// Returns the sender [`Address`].
    #[inline(always)]
    pub fn address(&self) -> &Address {
        &self.0
    }
}

impl Rent for SenderFeature {
    fn build_weighted_bytes(&self, builder: RentBuilder) -> RentBuilder {
        builder
            // Feature Type
            .data_field::<u8>()
            // Address
            .packable_data_field(&self.0)
    }
}

#[cfg(feature = "serde")]
mod dto {
    use serde::{Deserialize, Serialize};

    use super::*;

    #[derive(Serialize, Deserialize)]
    struct SenderFeatureDto {
        #[serde(rename = "type")]
        kind: u8,
        address: Address,
    }

    impl From<&SenderFeature> for SenderFeatureDto {
        fn from(value: &SenderFeature) -> Self {
            Self {
                kind: SenderFeature::KIND,
                address: value.0,
            }
        }
    }

    impl From<SenderFeatureDto> for SenderFeature {
        fn from(value: SenderFeatureDto) -> Self {
            Self(value.address)
        }
    }

    impl_serde_typed_dto!(SenderFeature, SenderFeatureDto, "sender feature");
}
