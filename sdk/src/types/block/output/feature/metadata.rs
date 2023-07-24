// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::{boxed::Box, vec::Vec};
use core::{ops::RangeInclusive, str::FromStr};

use packable::{bounded::BoundedU16, prefix::BoxedSlicePrefix};

use crate::types::block::Error;

pub(crate) type MetadataFeatureLength =
    BoundedU16<{ *MetadataFeature::LENGTH_RANGE.start() }, { *MetadataFeature::LENGTH_RANGE.end() }>;

/// Defines metadata, arbitrary binary data, that will be stored in the output.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, packable::Packable)]
#[packable(unpack_error = Error, with = |err| Error::InvalidMetadataFeatureLength(err.into_prefix_err().into()))]
pub struct MetadataFeature(
    // Binary data.
    BoxedSlicePrefix<u8, MetadataFeatureLength>,
);

impl TryFrom<Vec<u8>> for MetadataFeature {
    type Error = Error;

    fn try_from(data: Vec<u8>) -> Result<Self, Error> {
        data.into_boxed_slice().try_into()
    }
}

impl TryFrom<Box<[u8]>> for MetadataFeature {
    type Error = Error;

    fn try_from(data: Box<[u8]>) -> Result<Self, Error> {
        data.try_into().map(Self).map_err(Error::InvalidMetadataFeatureLength)
    }
}

impl FromStr for MetadataFeature {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(prefix_hex::decode::<Vec<u8>>(s).map_err(Error::Hex)?)
    }
}

impl MetadataFeature {
    /// The [`Feature`](crate::types::block::output::Feature) kind of [`MetadataFeature`].
    pub const KIND: u8 = 2;
    /// Valid lengths for a [`MetadataFeature`].
    pub const LENGTH_RANGE: RangeInclusive<u16> = 1..=8192;

    /// Creates a new [`MetadataFeature`].
    #[inline(always)]
    pub fn new(data: impl Into<Vec<u8>>) -> Result<Self, Error> {
        Self::try_from(data.into())
    }

    /// Returns the data.
    #[inline(always)]
    pub fn data(&self) -> &[u8] {
        &self.0
    }
}

impl core::fmt::Display for MetadataFeature {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", prefix_hex::encode(self.data()))
    }
}

impl core::fmt::Debug for MetadataFeature {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "MetadataFeature({self})")
    }
}

pub(crate) mod dto {
    use alloc::boxed::Box;

    use serde::{Deserialize, Serialize};

    use crate::utils::serde::prefix_hex_bytes;

    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    pub struct MetadataFeatureDto {
        #[serde(rename = "type")]
        pub kind: u8,
        #[serde(skip_serializing_if = "<[_]>::is_empty", default, with = "prefix_hex_bytes")]
        pub data: Box<[u8]>,
    }
}
