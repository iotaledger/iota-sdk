// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::{string::String, vec::Vec};
use core::convert::Infallible;

use crate::types::block::{
    address::AddressError,
    output::{
        feature::{
            BlockIssuerKeyCount, FeatureCount, MetadataFeatureEntryCount, MetadataFeatureKeyLength,
            MetadataFeatureValueLength, TagFeatureLength,
        },
        NativeTokenError,
    },
};

#[derive(Debug, PartialEq, Eq, derive_more::Display, derive_more::From)]
#[allow(missing_docs)]
pub enum FeatureError {
    #[display(fmt = "invalid feature kind: {_0}")]
    Kind(u8),
    #[display(fmt = "invalid feature count: {_0}")]
    Count(<FeatureCount as TryFrom<usize>>::Error),
    #[display(fmt = "invalid tag feature length {_0}")]
    TagFeatureLength(<TagFeatureLength as TryFrom<usize>>::Error),
    #[display(fmt = "invalid metadata feature: {_0}")]
    MetadataFeature(String),
    #[display(fmt = "invalid metadata feature entry count: {_0}")]
    MetadataFeatureEntryCount(<MetadataFeatureEntryCount as TryFrom<usize>>::Error),
    #[display(fmt = "invalid metadata feature key length: {_0}")]
    MetadataFeatureKeyLength(<MetadataFeatureKeyLength as TryFrom<usize>>::Error),
    #[display(fmt = "invalid metadata feature value length: {_0}")]
    MetadataFeatureValueLength(<MetadataFeatureValueLength as TryFrom<usize>>::Error),
    #[display(fmt = "features are not unique and/or sorted")]
    NotUniqueSorted,
    #[display(fmt = "disallowed feature at index {index} with kind {kind}")]
    Disallowed { index: usize, kind: u8 },
    #[display(fmt = "non graphic ASCII key: {_0:?}")]
    NonGraphicAsciiMetadataKey(Vec<u8>),
    #[display(fmt = "invalid block issuer key kind: {_0}")]
    InvalidBlockIssuerKeyKind(u8),
    #[display(fmt = "invalid block issuer key count: {_0}")]
    InvalidBlockIssuerKeyCount(<BlockIssuerKeyCount as TryFrom<usize>>::Error),
    #[display(fmt = "block issuer keys are not unique and/or sorted")]
    BlockIssuerKeysNotUniqueSorted,
    #[from]
    NativeToken(NativeTokenError),
    #[from]
    Address(AddressError),
}

#[cfg(feature = "std")]
impl std::error::Error for FeatureError {}

impl From<Infallible> for FeatureError {
    fn from(error: Infallible) -> Self {
        match error {}
    }
}
