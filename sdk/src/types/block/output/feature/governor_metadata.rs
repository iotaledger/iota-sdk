// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::{boxed::Box, string::String, vec::Vec};
use core::{ops::RangeInclusive, str::FromStr};

use packable::{bounded::BoundedU16, prefix::BoxedSlicePrefix};

use crate::types::block::Error;

pub(crate) type GovernorMetadataFeatureLength =
    BoundedU16<{ *GovernorMetadataFeature::LENGTH_RANGE.start() }, { *GovernorMetadataFeature::LENGTH_RANGE.end() }>;

/// Defines governor metadata, arbitrary binary data, that will be stored in the anchor output.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, packable::Packable)]
#[packable(unpack_error = Error, with = |err| Error::InvalidGovernorMetadataFeatureLength(err.into_prefix_err().into()))]
pub struct GovernorMetadataFeature(
    // Binary data.
    pub(crate) BoxedSlicePrefix<u8, GovernorMetadataFeatureLength>,
);

macro_rules! impl_from_vec {
    ($type:ty) => {
        impl TryFrom<$type> for GovernorMetadataFeature {
            type Error = Error;

            fn try_from(value: $type) -> Result<Self, Self::Error> {
                Vec::<u8>::from(value).try_into()
            }
        }
    };
}
impl_from_vec!(&str);
impl_from_vec!(String);
impl_from_vec!(&[u8]);

impl<const N: usize> TryFrom<[u8; N]> for GovernorMetadataFeature {
    type Error = Error;

    fn try_from(value: [u8; N]) -> Result<Self, Self::Error> {
        value.to_vec().try_into()
    }
}

impl TryFrom<Vec<u8>> for GovernorMetadataFeature {
    type Error = Error;

    fn try_from(data: Vec<u8>) -> Result<Self, Error> {
        data.into_boxed_slice().try_into()
    }
}

impl TryFrom<Box<[u8]>> for GovernorMetadataFeature {
    type Error = Error;

    fn try_from(data: Box<[u8]>) -> Result<Self, Error> {
        data.try_into()
            .map(Self)
            .map_err(Error::InvalidGovernorMetadataFeatureLength)
    }
}

impl FromStr for GovernorMetadataFeature {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(prefix_hex::decode::<Vec<u8>>(s).map_err(Error::Hex)?)
    }
}

impl GovernorMetadataFeature {
    /// The [`Feature`](crate::types::block::output::Feature) kind of [`GovernorMetadataFeature`].
    pub const KIND: u8 = 3;
    /// Valid lengths for a [`GovernorMetadataFeature`].
    pub const LENGTH_RANGE: RangeInclusive<u16> = 1..=8192;

    /// Creates a new [`GovernorMetadataFeature`].
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

impl core::fmt::Display for GovernorMetadataFeature {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", prefix_hex::encode(self.data()))
    }
}

impl core::fmt::Debug for GovernorMetadataFeature {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "GovernorMetadataFeature({self})")
    }
}

#[cfg(feature = "serde")]
pub(crate) mod dto {
    use alloc::borrow::Cow;

    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::utils::serde::cow_boxed_slice_prefix_hex_bytes;

    #[derive(Serialize, Deserialize)]
    struct GovernorMetadataFeatureDto<'a> {
        #[serde(rename = "type")]
        kind: u8,
        #[serde(with = "cow_boxed_slice_prefix_hex_bytes")]
        data: Cow<'a, BoxedSlicePrefix<u8, GovernorMetadataFeatureLength>>,
    }

    impl<'a> From<&'a GovernorMetadataFeature> for GovernorMetadataFeatureDto<'a> {
        fn from(value: &'a GovernorMetadataFeature) -> Self {
            Self {
                kind: GovernorMetadataFeature::KIND,
                data: Cow::Borrowed(&value.0),
            }
        }
    }

    impl<'a> From<GovernorMetadataFeatureDto<'a>> for GovernorMetadataFeature {
        fn from(value: GovernorMetadataFeatureDto<'a>) -> Self {
            Self(value.data.into_owned())
        }
    }

    crate::impl_serde_typed_dto!(
        GovernorMetadataFeature,
        GovernorMetadataFeatureDto<'_>,
        "governor metadata feature"
    );
}
