// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Module describing the tagged data payload.

use alloc::boxed::Box;
use core::ops::RangeInclusive;

use packable::{
    bounded::{BoundedU32, BoundedU8},
    prefix::BoxedSlicePrefix,
    Packable, PackableExt,
};

use crate::types::block::{
    protocol::{WorkScore, WorkScoreParameters},
    Error,
};

pub(crate) type TagLength =
    BoundedU8<{ *TaggedDataPayload::TAG_LENGTH_RANGE.start() }, { *TaggedDataPayload::TAG_LENGTH_RANGE.end() }>;
pub(crate) type TaggedDataLength =
    BoundedU32<{ *TaggedDataPayload::DATA_LENGTH_RANGE.start() }, { *TaggedDataPayload::DATA_LENGTH_RANGE.end() }>;

/// A payload which holds a tag and associated data.
#[derive(Clone, Eq, PartialEq, Packable)]
#[packable(unpack_error = Error)]
pub struct TaggedDataPayload {
    #[packable(unpack_error_with = |err| Error::InvalidTagLength(err.into_prefix_err().into()))]
    tag: BoxedSlicePrefix<u8, TagLength>,
    #[packable(unpack_error_with = |err| Error::InvalidTaggedDataLength(err.into_prefix_err().into()))]
    data: BoxedSlicePrefix<u8, TaggedDataLength>,
}

impl TaggedDataPayload {
    /// The [`Payload`](crate::types::block::payload::Payload) kind of a [`TaggedDataPayload`].
    pub const KIND: u8 = 0;
    /// Valid length range for the tag.
    pub const TAG_LENGTH_RANGE: RangeInclusive<u8> = 0..=64;
    /// Valid length range for the data.
    pub const DATA_LENGTH_RANGE: RangeInclusive<u32> = 0..=8192;

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

impl WorkScore for TaggedDataPayload {
    fn work_score(&self, params: WorkScoreParameters) -> u32 {
        // 1 byte for the payload kind
        (1 + self.packed_len() as u32) * params.data_byte()
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

#[cfg(feature = "serde")]
pub mod dto {
    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::{types::block::Error, utils::serde::prefix_hex_bytes};

    /// The payload type to define a tagged data payload.
    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    struct TaggedDataPayloadDto {
        #[serde(rename = "type")]
        kind: u8,
        #[serde(skip_serializing_if = "<[_]>::is_empty", default, with = "prefix_hex_bytes")]
        tag: Box<[u8]>,
        #[serde(skip_serializing_if = "<[_]>::is_empty", default, with = "prefix_hex_bytes")]
        data: Box<[u8]>,
    }

    impl From<&TaggedDataPayload> for TaggedDataPayloadDto {
        fn from(value: &TaggedDataPayload) -> Self {
            Self {
                kind: TaggedDataPayload::KIND,
                tag: value.tag().into(),
                data: value.data().into(),
            }
        }
    }

    impl TryFrom<TaggedDataPayloadDto> for TaggedDataPayload {
        type Error = Error;

        fn try_from(value: TaggedDataPayloadDto) -> Result<Self, Self::Error> {
            Self::new(value.tag, value.data)
        }
    }

    crate::impl_serde_typed_dto!(TaggedDataPayload, TaggedDataPayloadDto, "tagged data payload");
}
