// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::{boxed::Box, vec::Vec};
use core::ops::RangeInclusive;

use packable::{bounded::BoundedU8, prefix::BoxedSlicePrefix};

use crate::types::block::{
    output::{feature::FeatureError, StorageScore},
    protocol::WorkScore,
};

pub(crate) type TagFeatureLength =
    BoundedU8<{ *TagFeature::LENGTH_RANGE.start() }, { *TagFeature::LENGTH_RANGE.end() }>;

/// Makes it possible to tag outputs with an index, so they can be retrieved through an indexer API.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, packable::Packable)]
#[packable(unpack_error = FeatureError, with = |e| FeatureError::TagFeatureLength(e.into_prefix_err().into()))]
pub struct TagFeature(
    // Binary tag.
    pub(crate) BoxedSlicePrefix<u8, TagFeatureLength>,
);

impl TagFeature {
    /// The [`Feature`](crate::types::block::output::Feature) kind of an [`TagFeature`].
    pub const KIND: u8 = 4;
    /// Valid lengths for an [`TagFeature`].
    pub const LENGTH_RANGE: RangeInclusive<u8> = 1..=64;

    /// Creates a new [`TagFeature`].
    #[inline(always)]
    pub fn new(tag: impl Into<Vec<u8>>) -> Result<Self, FeatureError> {
        Self::try_from(tag.into())
    }

    /// Returns the tag.
    #[inline(always)]
    pub fn tag(&self) -> &[u8] {
        &self.0
    }
}

impl StorageScore for TagFeature {}

impl WorkScore for TagFeature {}

impl TryFrom<Vec<u8>> for TagFeature {
    type Error = FeatureError;

    fn try_from(tag: Vec<u8>) -> Result<Self, Self::Error> {
        tag.into_boxed_slice().try_into()
    }
}

impl TryFrom<Box<[u8]>> for TagFeature {
    type Error = FeatureError;

    fn try_from(tag: Box<[u8]>) -> Result<Self, Self::Error> {
        tag.try_into().map(Self).map_err(FeatureError::TagFeatureLength)
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

#[cfg(feature = "serde")]
pub(crate) mod dto {
    use alloc::borrow::Cow;

    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::utils::serde::cow_boxed_slice_prefix_hex_bytes;

    #[derive(Serialize, Deserialize)]
    struct TagFeatureDto<'a> {
        #[serde(rename = "type")]
        kind: u8,
        #[serde(with = "cow_boxed_slice_prefix_hex_bytes")]
        tag: Cow<'a, BoxedSlicePrefix<u8, TagFeatureLength>>,
    }

    impl<'a> From<&'a TagFeature> for TagFeatureDto<'a> {
        fn from(value: &'a TagFeature) -> Self {
            Self {
                kind: TagFeature::KIND,
                tag: Cow::Borrowed(&value.0),
            }
        }
    }

    impl<'a> From<TagFeatureDto<'a>> for TagFeature {
        fn from(value: TagFeatureDto<'a>) -> Self {
            Self(value.tag.into_owned())
        }
    }

    crate::impl_serde_typed_dto!(TagFeature, TagFeatureDto<'_>, "tag feature");
}
