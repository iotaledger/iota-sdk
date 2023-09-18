// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod block_issuer;
mod issuer;
mod metadata;
mod sender;
mod staking;
mod tag;

use alloc::{collections::BTreeSet, vec::Vec};

use bitflags::bitflags;
use derive_more::{Deref, From};
use packable::{
    bounded::BoundedU8,
    prefix::BTreeSetPrefix,
    set::{UnpackOrderedSetError, UnpackSetError},
    Packable,
};

#[cfg(feature = "irc_27")]
pub use self::metadata::irc_27::{Attribute, Irc27Metadata};
#[cfg(feature = "irc_30")]
pub use self::metadata::irc_30::Irc30Metadata;
pub(crate) use self::{block_issuer::BlockIssuerKeyCount, metadata::MetadataFeatureLength, tag::TagFeatureLength};
pub use self::{
    block_issuer::{BlockIssuerFeature, BlockIssuerKey, BlockIssuerKeys, Ed25519BlockIssuerKey},
    issuer::IssuerFeature,
    metadata::MetadataFeature,
    sender::SenderFeature,
    staking::StakingFeature,
    tag::TagFeature,
};
use crate::types::block::{create_bitflags, Error};

///
#[derive(Clone, Eq, PartialEq, Hash, From, Packable)]
#[packable(unpack_error = Error)]
#[packable(tag_type = u8, with_error = Error::InvalidFeatureKind)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(untagged))]
pub enum Feature {
    /// A sender feature.
    #[packable(tag = SenderFeature::KIND)]
    Sender(SenderFeature),
    /// An issuer feature.
    #[packable(tag = IssuerFeature::KIND)]
    Issuer(IssuerFeature),
    /// A metadata feature.
    #[packable(tag = MetadataFeature::KIND)]
    Metadata(MetadataFeature),
    /// A tag feature.
    #[packable(tag = TagFeature::KIND)]
    Tag(TagFeature),
    /// A block issuer feature.
    #[packable(tag = BlockIssuerFeature::KIND)]
    BlockIssuer(BlockIssuerFeature),
    /// A staking feature.
    #[packable(tag = StakingFeature::KIND)]
    Staking(StakingFeature),
}

impl PartialOrd for Feature {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Feature {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.kind().cmp(&other.kind())
    }
}

impl core::borrow::Borrow<u8> for Feature {
    fn borrow(&self) -> &u8 {
        &self.ord_kind()
    }
}

impl core::fmt::Debug for Feature {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Sender(feature) => feature.fmt(f),
            Self::Issuer(feature) => feature.fmt(f),
            Self::Metadata(feature) => feature.fmt(f),
            Self::Tag(feature) => feature.fmt(f),
            Self::BlockIssuer(feature) => feature.fmt(f),
            Self::Staking(feature) => feature.fmt(f),
        }
    }
}

impl Feature {
    /// Return the output kind of an `Output`.
    pub fn kind(&self) -> u8 {
        *self.ord_kind()
    }

    fn ord_kind<'a>(&'a self) -> &'a u8 {
        match self {
            Self::Sender(_) => &SenderFeature::KIND,
            Self::Issuer(_) => &IssuerFeature::KIND,
            Self::Metadata(_) => &MetadataFeature::KIND,
            Self::Tag(_) => &TagFeature::KIND,
            Self::BlockIssuer(_) => &BlockIssuerFeature::KIND,
            Self::Staking(_) => &StakingFeature::KIND,
        }
    }

    /// Returns the [`FeatureFlags`] for the given [`Feature`].
    pub fn flag(&self) -> FeatureFlags {
        match self {
            Self::Sender(_) => FeatureFlags::SENDER,
            Self::Issuer(_) => FeatureFlags::ISSUER,
            Self::Metadata(_) => FeatureFlags::METADATA,
            Self::Tag(_) => FeatureFlags::TAG,
            Self::BlockIssuer(_) => FeatureFlags::BLOCK_ISSUER,
            Self::Staking(_) => FeatureFlags::STAKING,
        }
    }

    /// Checks whether the feature is a [`SenderFeature`].
    pub fn is_sender(&self) -> bool {
        matches!(self, Self::Sender(_))
    }

    /// Gets the feature as an actual [`SenderFeature`].
    /// NOTE: Will panic if the feature is not a [`SenderFeature`].
    pub fn as_sender(&self) -> &SenderFeature {
        if let Self::Sender(feature) = self {
            feature
        } else {
            panic!("invalid downcast of non-SenderFeature");
        }
    }

    /// Checks whether the feature is an [`IssuerFeature`].
    pub fn is_issuer(&self) -> bool {
        matches!(self, Self::Issuer(_))
    }

    /// Gets the feature as an actual [`IssuerFeature`].
    /// NOTE: Will panic if the feature is not an [`IssuerFeature`].
    pub fn as_issuer(&self) -> &IssuerFeature {
        if let Self::Issuer(feature) = self {
            feature
        } else {
            panic!("invalid downcast of non-IssuerFeature");
        }
    }

    /// Checks whether the feature is a [`MetadataFeature`].
    pub fn is_metadata(&self) -> bool {
        matches!(self, Self::Metadata(_))
    }

    /// Gets the feature as an actual [`MetadataFeature`].
    /// NOTE: Will panic if the feature is not a [`MetadataFeature`].
    pub fn as_metadata(&self) -> &MetadataFeature {
        if let Self::Metadata(feature) = self {
            feature
        } else {
            panic!("invalid downcast of non-MetadataFeature");
        }
    }

    /// Checks whether the feature is a [`TagFeature`].
    pub fn is_tag(&self) -> bool {
        matches!(self, Self::Tag(_))
    }

    /// Gets the feature as an actual [`TagFeature`].
    /// NOTE: Will panic if the feature is not a [`TagFeature`].
    pub fn as_tag(&self) -> &TagFeature {
        if let Self::Tag(feature) = self {
            feature
        } else {
            panic!("invalid downcast of non-TagFeature");
        }
    }

    /// Checks whether the feature is a [`BlockIssuerFeature`].
    pub fn is_block_issuer(&self) -> bool {
        matches!(self, Self::BlockIssuer(_))
    }

    /// Gets the feature as an actual [`BlockIssuerFeature`].
    /// NOTE: Will panic if the feature is not a [`BlockIssuerFeature`].
    pub fn as_block_issuer(&self) -> &BlockIssuerFeature {
        if let Self::BlockIssuer(feature) = self {
            feature
        } else {
            panic!("invalid downcast of non-BlockIssuerFeature");
        }
    }

    /// Checks whether the feature is a [`StakingFeature`].
    pub fn is_staking(&self) -> bool {
        matches!(self, Self::Staking(_))
    }

    /// Gets the feature as an actual [`StakingFeature`].
    /// NOTE: Will panic if the feature is not a [`StakingFeature`].
    pub fn as_staking(&self) -> &StakingFeature {
        if let Self::Staking(feature) = self {
            feature
        } else {
            panic!("invalid downcast of non-StakingFeature");
        }
    }
}

create_bitflags!(
    /// A bitflags-based representation of the set of active [`Feature`]s.
    pub FeatureFlags,
    u16,
    [
        (SENDER, SenderFeature),
        (ISSUER, IssuerFeature),
        (METADATA, MetadataFeature),
        (TAG, TagFeature),
        (BLOCK_ISSUER, BlockIssuerFeature),
        (STAKING, StakingFeature),
    ]
);

pub(crate) type FeatureCount = BoundedU8<0, { Features::COUNT_MAX }>;

///
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Deref, Packable)]
#[packable(unpack_error = Error, with = map_features_set_error)]
pub struct Features(BTreeSetPrefix<Feature, FeatureCount>);

fn map_features_set_error<T, P>(error: UnpackOrderedSetError<T, Error, P>) -> Error
where
    <FeatureCount as TryFrom<usize>>::Error: From<P>,
{
    match error {
        UnpackOrderedSetError::Set(e) => match e {
            UnpackSetError::DuplicateItem(_) => Error::FeaturesNotUniqueSorted,
            UnpackSetError::Item(e) => e,
            UnpackSetError::Prefix(p) => Error::InvalidFeatureCount(p.into()),
        },
        UnpackOrderedSetError::Unordered => Error::FeaturesNotUniqueSorted,
    }
}

impl TryFrom<Vec<Feature>> for Features {
    type Error = Error;

    #[inline(always)]
    fn try_from(features: Vec<Feature>) -> Result<Self, Self::Error> {
        Self::from_vec(features)
    }
}

impl TryFrom<BTreeSet<Feature>> for Features {
    type Error = Error;

    #[inline(always)]
    fn try_from(features: BTreeSet<Feature>) -> Result<Self, Self::Error> {
        Self::from_set(features)
    }
}

impl IntoIterator for Features {
    type Item = Feature;
    type IntoIter = alloc::collections::btree_set::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        BTreeSet::from(self.0).into_iter()
    }
}

impl Features {
    ///
    pub const COUNT_MAX: u8 = 5;

    /// Creates a new [`Features`] from a vec.
    pub fn from_vec(features: Vec<Feature>) -> Result<Self, Error> {
        Ok(Self(
            features
                .into_iter()
                .collect::<BTreeSet<_>>()
                .try_into()
                .map_err(Error::InvalidFeatureCount)?,
        ))
    }

    /// Creates a new [`Features`] from an ordered set.
    pub fn from_set(features: BTreeSet<Feature>) -> Result<Self, Error> {
        Ok(Self(features.try_into().map_err(Error::InvalidFeatureCount)?))
    }

    /// Gets the underlying set.
    pub fn as_set(&self) -> &BTreeSet<Feature> {
        &self.0
    }

    /// Gets a reference to a [`SenderFeature`], if any.
    pub fn sender(&self) -> Option<&SenderFeature> {
        self.get(&SenderFeature::KIND).map(Feature::as_sender)
    }

    /// Gets a reference to a [`IssuerFeature`], if any.
    pub fn issuer(&self) -> Option<&IssuerFeature> {
        self.get(&IssuerFeature::KIND).map(Feature::as_issuer)
    }

    /// Gets a reference to a [`MetadataFeature`], if any.
    pub fn metadata(&self) -> Option<&MetadataFeature> {
        self.get(&MetadataFeature::KIND).map(Feature::as_metadata)
    }

    /// Gets a reference to a [`TagFeature`], if any.
    pub fn tag(&self) -> Option<&TagFeature> {
        self.get(&TagFeature::KIND).map(Feature::as_tag)
    }

    /// Gets a reference to a [`BlockIssuerFeature`], if any.
    pub fn block_issuer(&self) -> Option<&BlockIssuerFeature> {
        self.get(&BlockIssuerFeature::KIND).map(Feature::as_block_issuer)
    }

    /// Gets a reference to a [`StakingFeature`], if any.
    pub fn staking(&self) -> Option<&StakingFeature> {
        self.get(&StakingFeature::KIND).map(Feature::as_staking)
    }
}

pub(crate) fn verify_allowed_features(features: &Features, allowed_features: FeatureFlags) -> Result<(), Error> {
    for (index, feature) in features.iter().enumerate() {
        if !allowed_features.contains(feature.flag()) {
            return Err(Error::UnallowedFeature {
                index,
                kind: feature.kind(),
            });
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn all_flags_present() {
        assert_eq!(
            FeatureFlags::ALL_FLAGS,
            &[
                FeatureFlags::SENDER,
                FeatureFlags::ISSUER,
                FeatureFlags::METADATA,
                FeatureFlags::TAG,
                FeatureFlags::BLOCK_ISSUER,
                FeatureFlags::STAKING
            ]
        );
    }
}
