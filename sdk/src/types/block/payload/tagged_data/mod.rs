// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Module describing the tagged data payload.

use alloc::vec::Vec;
use core::ops::RangeInclusive;

use packable::{
    bounded::{BoundedU32, BoundedU8},
    prefix::BoxedSlicePrefix,
    Packable,
};

use crate::types::block::{Block, Error};

pub(crate) type TagLength =
    BoundedU8<{ *TaggedDataPayload::TAG_LENGTH_RANGE.start() }, { *TaggedDataPayload::TAG_LENGTH_RANGE.end() }>;
pub(crate) type TaggedDataLength =
    BoundedU32<{ *TaggedDataPayload::DATA_LENGTH_RANGE.start() }, { *TaggedDataPayload::DATA_LENGTH_RANGE.end() }>;

/// A payload which holds a tag and associated data.
#[derive(Clone, Eq, PartialEq, Packable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[packable(unpack_error = Error)]
pub struct TaggedDataPayload {
    #[packable(unpack_error_with = |err| Error::InvalidTagLength(err.into_prefix_err().into()))]
    tag: BoxedSlicePrefix<u8, TagLength>,
    #[packable(unpack_error_with = |err| Error::InvalidTaggedDataLength(err.into_prefix_err().into()))]
    data: BoxedSlicePrefix<u8, TaggedDataLength>,
}

impl TaggedDataPayload {
    /// The payload kind of a [`TaggedDataPayload`].
    pub const KIND: u32 = 5;
    /// Valid lengths for the tag.
    pub const TAG_LENGTH_RANGE: RangeInclusive<u8> = 0..=64;
    /// Valid lengths for the data.
    // Less than max block length, because of the other fields in the block and payload kind, tagged payload field
    // lengths.
    pub const DATA_LENGTH_RANGE: RangeInclusive<u32> = 0..=(Block::LENGTH_MAX - Block::LENGTH_MIN - 9) as u32;

    /// Creates a new [`TaggedDataPayload`].
    pub fn new(tag: impl Into<Box<[u8]>>, data: impl Into<Box<[u8]>>) -> Result<Self, Error> {
        Ok(Self {
            tag: tag.into().try_into().map_err(Error::InvalidTagLength)?,
            data: data.into().try_into().map_err(Error::InvalidTaggedDataLength)?,
        })
    }

    /// Returns the tag of a [`TaggedDataPayload`].
    pub fn tag(&self) -> &[u8] {
        &self.tag
    }

    /// Returns the data of a [`TaggedDataPayload`].
    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

impl core::fmt::Debug for TaggedDataPayload {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("TaggedDataPayload")
            .field("tag", &prefix_hex::encode(self.tag()))
            .field("data", &prefix_hex::encode(self.data()))
            .finish()
    }
}

#[allow(missing_docs)]
pub mod dto {
    use alloc::string::String;

    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::types::block::Error;

    /// The payload type to define a tagged data payload.
    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    pub struct TaggedDataPayloadDto {
        #[serde(rename = "type")]
        pub kind: u32,
        #[serde(skip_serializing_if = "String::is_empty", default)]
        pub tag: String,
        #[serde(skip_serializing_if = "String::is_empty", default)]
        pub data: String,
    }

    impl From<&TaggedDataPayload> for TaggedDataPayloadDto {
        fn from(value: &TaggedDataPayload) -> Self {
            Self {
                kind: TaggedDataPayload::KIND,
                tag: prefix_hex::encode(value.tag()),
                data: prefix_hex::encode(value.data()),
            }
        }
    }

    impl TryFrom<&TaggedDataPayloadDto> for TaggedDataPayload {
        type Error = Error;

        fn try_from(value: &TaggedDataPayloadDto) -> Result<Self, Self::Error> {
            Self::new(
                if !value.tag.is_empty() {
                    prefix_hex::decode(&value.tag).map_err(|_| Error::InvalidField("tag"))?
                } else {
                    Vec::new()
                },
                if !value.data.is_empty() {
                    prefix_hex::decode(&value.data).map_err(|_| Error::InvalidField("data"))?
                } else {
                    Vec::new()
                },
            )
        }
    }
}
