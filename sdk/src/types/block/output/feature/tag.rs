// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::{boxed::Box, vec::Vec};
use core::ops::RangeInclusive;

use packable::{bounded::BoundedU8, prefix::BoxedSlicePrefix};

use crate::types::block::Error;

pub(crate) type TagFeatureLength =
    BoundedU8<{ *TagFeature::LENGTH_RANGE.start() }, { *TagFeature::LENGTH_RANGE.end() }>;

/// Makes it possible to tag outputs with an index, so they can be retrieved through an indexer API.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, packable::Packable)]
#[packable(unpack_error = Error, with = |e| Error::InvalidTagFeatureLength(e.into_prefix_err().into()))]
pub struct TagFeature(
    // Binary tag.
    BoxedSlicePrefix<u8, TagFeatureLength>,
);

impl TryFrom<Vec<u8>> for TagFeature {
    type Error = Error;

    fn try_from(tag: Vec<u8>) -> Result<Self, Error> {
        tag.into_boxed_slice().try_into()
    }
}

impl TryFrom<Box<[u8]>> for TagFeature {
    type Error = Error;

    fn try_from(tag: Box<[u8]>) -> Result<Self, Error> {
        tag.try_into().map(Self).map_err(Error::InvalidTagFeatureLength)
    }
}

impl TagFeature {
    /// The [`Feature`](crate::types::block::output::Feature) kind of an [`TagFeature`].
    pub const KIND: u8 = 3;
    /// Valid lengths for an [`TagFeature`].
    pub const LENGTH_RANGE: RangeInclusive<u8> = 1..=64;

    /// Creates a new [`TagFeature`].
    #[inline(always)]
    pub fn new(tag: impl Into<Vec<u8>>) -> Result<Self, Error> {
        Self::try_from(tag.into())
    }

    /// Returns the tag.
    #[inline(always)]
    pub fn tag(&self) -> &[u8] {
        &self.0
    }
}

impl core::fmt::Display for TagFeature {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", prefix_hex::encode(self.tag()))
    }
}

impl core::fmt::Debug for TagFeature {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "TagFeature({self})")
    }
}

pub(crate) mod dto {
    use alloc::boxed::Box;

    use serde::{Deserialize, Serialize};

    use crate::utils::serde::prefix_hex_bytes;

    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    pub struct TagFeatureDto {
        #[serde(rename = "type")]
        pub kind: u8,
        #[serde(skip_serializing_if = "<[_]>::is_empty", default, with = "prefix_hex_bytes")]
        pub tag: Box<[u8]>,
    }
}
