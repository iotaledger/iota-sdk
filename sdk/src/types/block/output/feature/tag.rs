// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::{boxed::Box, vec::Vec};
use core::ops::RangeInclusive;

use packable::{bounded::BoundedU8, prefix::BoxedSlicePrefix};

use crate::types::block::{
    output::{rent::RentBuilder, Rent},
    Error,
};

pub(crate) type TagFeatureLength =
    BoundedU8<{ *TagFeature::LENGTH_RANGE.start() }, { *TagFeature::LENGTH_RANGE.end() }>;

/// Makes it possible to tag outputs with an index, so they can be retrieved through an indexer API.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, packable::Packable)]
#[packable(unpack_error = Error, with = |e| Error::InvalidTagFeatureLength(e.into_prefix_err().into()))]
pub struct TagFeature(
    // Binary tag.
    pub(crate) BoxedSlicePrefix<u8, TagFeatureLength>,
);

impl Rent for TagFeature {
    fn build_weighted_bytes(&self, builder: &mut RentBuilder) {
        // Feature Type
        builder
            .data_field::<u8>()
            // Tag
            .packable_data_field(&self.0);
    }
}

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

mod dto {
    use alloc::borrow::Cow;

    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::utils::serde::cow_boxed_slice_prefix;

    #[derive(Serialize, Deserialize)]
    struct TagFeatureDto<'a> {
        #[serde(rename = "type")]
        kind: u8,
        #[serde(with = "cow_boxed_slice_prefix")]
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

    impl_serde_typed_dto!(TagFeature, TagFeatureDto<'_>, "tag feature");
}
