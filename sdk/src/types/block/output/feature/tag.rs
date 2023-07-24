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
    pub(crate) BoxedSlicePrefix<u8, TagFeatureLength>,
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

mod dto {
    use alloc::borrow::Cow;

    use serde::{Deserialize, Serialize};

    use super::*;

    #[derive(Serialize, Deserialize)]
    struct TagFeatureDto<'a> {
        #[serde(rename = "type")]
        kind: u8,
        #[serde(deserialize_with = "deserialize_tag")]
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

    fn deserialize_tag<'de, 'a, D>(d: D) -> Result<Cow<'a, BoxedSlicePrefix<u8, TagFeatureLength>>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Cow::Owned(crate::utils::serde::boxed_slice_prefix::deserialize(d)?))
    }

    impl<'de> Deserialize<'de> for TagFeature {
        fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            let dto = TagFeatureDto::deserialize(d)?;
            if dto.kind != Self::KIND {
                return Err(serde::de::Error::custom(format!(
                    "invalid tag feature type: expected {}, found {}",
                    Self::KIND,
                    dto.kind
                )));
            }
            Ok(dto.into())
        }
    }

    impl Serialize for TagFeature {
        fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            TagFeatureDto::from(self).serialize(s)
        }
    }
}
