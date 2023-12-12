// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod block_issuer;
mod issuer;
mod metadata;
mod native_token;
mod sender;
mod staking;
mod tag;

use alloc::{boxed::Box, collections::BTreeSet, vec::Vec};

use bitflags::bitflags;
use derive_more::{Deref, From};
use iterator_sorted::is_unique_sorted;
use packable::{bounded::BoundedU8, prefix::BoxedSlicePrefix, Packable};

#[cfg(feature = "irc_27")]
pub use self::metadata::irc_27::{Attribute, Irc27Metadata};
#[cfg(feature = "irc_30")]
pub use self::metadata::irc_30::Irc30Metadata;
pub(crate) use self::{
    block_issuer::BlockIssuerKeyCount,
    metadata::{MetadataFeatureKeyLength, MetadataFeatureLength, MetadataFeatureValueLength},
    tag::TagFeatureLength,
};
pub use self::{
    block_issuer::{BlockIssuerFeature, BlockIssuerKey, BlockIssuerKeys, Ed25519BlockIssuerKey},
    issuer::IssuerFeature,
    metadata::MetadataFeature,
    native_token::NativeTokenFeature,
    sender::SenderFeature,
    staking::StakingFeature,
    tag::TagFeature,
};
use crate::types::block::{
    output::{StorageScore, StorageScoreParameters},
    protocol::{WorkScore, WorkScoreParameters},
    Error,
};

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
    /// A native token feature.
    #[packable(tag = NativeTokenFeature::KIND)]
    NativeToken(NativeTokenFeature),
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

impl StorageScore for Feature {
    fn storage_score(&self, params: StorageScoreParameters) -> u64 {
        match self {
            Self::Sender(sender) => sender.storage_score(params),
            Self::Issuer(issuer) => issuer.storage_score(params),
            Self::Metadata(metadata) => metadata.storage_score(params),
            Self::Tag(tag) => tag.storage_score(params),
            Self::NativeToken(native_token) => native_token.storage_score(params),
            Self::BlockIssuer(block_issuer) => block_issuer.storage_score(params),
            Self::Staking(staking) => staking.storage_score(params),
        }
    }
}

impl WorkScore for Feature {
    fn work_score(&self, params: WorkScoreParameters) -> u32 {
        match self {
            Self::Sender(sender) => sender.work_score(params),
            Self::Issuer(issuer) => issuer.work_score(params),
            Self::Metadata(metadata) => metadata.work_score(params),
            Self::Tag(tag) => tag.work_score(params),
            Self::NativeToken(native_token) => native_token.work_score(params),
            Self::BlockIssuer(block_issuer) => block_issuer.work_score(params),
            Self::Staking(staking) => staking.work_score(params),
        }
    }
}

impl core::fmt::Debug for Feature {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Sender(feature) => feature.fmt(f),
            Self::Issuer(feature) => feature.fmt(f),
            Self::Metadata(feature) => feature.fmt(f),
            Self::Tag(feature) => feature.fmt(f),
            Self::NativeToken(feature) => feature.fmt(f),
            Self::BlockIssuer(feature) => feature.fmt(f),
            Self::Staking(feature) => feature.fmt(f),
        }
    }
}

impl Feature {
    /// Return the output kind of an `Output`.
    pub fn kind(&self) -> u8 {
        match self {
            Self::Sender(_) => SenderFeature::KIND,
            Self::Issuer(_) => IssuerFeature::KIND,
            Self::Metadata(_) => MetadataFeature::KIND,
            Self::Tag(_) => TagFeature::KIND,
            Self::NativeToken(_) => NativeTokenFeature::KIND,
            Self::BlockIssuer(_) => BlockIssuerFeature::KIND,
            Self::Staking(_) => StakingFeature::KIND,
        }
    }

    /// Returns the [`FeatureFlags`] for the given [`Feature`].
    pub fn flag(&self) -> FeatureFlags {
        match self {
            Self::Sender(_) => FeatureFlags::SENDER,
            Self::Issuer(_) => FeatureFlags::ISSUER,
            Self::Metadata(_) => FeatureFlags::METADATA,
            Self::Tag(_) => FeatureFlags::TAG,
            Self::NativeToken(_) => FeatureFlags::NATIVE_TOKEN,
            Self::BlockIssuer(_) => FeatureFlags::BLOCK_ISSUER,
            Self::Staking(_) => FeatureFlags::STAKING,
        }
    }

    crate::def_is_as_opt!(Feature: Sender, Issuer, Metadata, Tag, NativeToken, BlockIssuer, Staking);
}

crate::create_bitflags!(
    /// A bitflags-based representation of the set of active [`Feature`]s.
    pub FeatureFlags,
    u16,
    [
        (SENDER, SenderFeature),
        (ISSUER, IssuerFeature),
        (METADATA, MetadataFeature),
        (TAG, TagFeature),
        (NATIVE_TOKEN, NativeTokenFeature),
        (BLOCK_ISSUER, BlockIssuerFeature),
        (STAKING, StakingFeature),
    ]
);

pub(crate) type FeatureCount = BoundedU8<0, { FeatureFlags::ALL_FLAGS.len() as u8 }>;

///
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Deref, Packable)]
#[packable(unpack_error = Error, with = |e| e.unwrap_item_err_or_else(|p| Error::InvalidFeatureCount(p.into())))]
pub struct Features(#[packable(verify_with = verify_unique_sorted)] BoxedSlicePrefix<Feature, FeatureCount>);

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
    type IntoIter = alloc::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        Vec::from(Into::<Box<[Feature]>>::into(self.0)).into_iter()
    }
}

impl Features {
    /// Creates a new [`Features`] from a vec.
    pub fn from_vec(features: Vec<Feature>) -> Result<Self, Error> {
        let mut features = BoxedSlicePrefix::<Feature, FeatureCount>::try_from(features.into_boxed_slice())
            .map_err(Error::InvalidFeatureCount)?;

        features.sort_by_key(Feature::kind);
        // Sort is obviously fine now but uniqueness still needs to be checked.
        verify_unique_sorted::<true>(&features)?;

        Ok(Self(features))
    }

    /// Creates a new [`Features`] from an ordered set.
    pub fn from_set(features: BTreeSet<Feature>) -> Result<Self, Error> {
        Ok(Self(
            features
                .into_iter()
                .collect::<Box<[_]>>()
                .try_into()
                .map_err(Error::InvalidFeatureCount)?,
        ))
    }

    /// Gets a reference to a [`Feature`] from a feature kind, if any.
    #[inline(always)]
    pub fn get(&self, key: u8) -> Option<&Feature> {
        self.0
            .binary_search_by_key(&key, Feature::kind)
            // PANIC: indexation is fine since the index has been found.
            .map(|index| &self.0[index])
            .ok()
    }

    /// Gets a reference to a [`SenderFeature`], if any.
    pub fn sender(&self) -> Option<&SenderFeature> {
        self.get(SenderFeature::KIND).map(Feature::as_sender)
    }

    /// Gets a reference to a [`IssuerFeature`], if any.
    pub fn issuer(&self) -> Option<&IssuerFeature> {
        self.get(IssuerFeature::KIND).map(Feature::as_issuer)
    }

    /// Gets a reference to a [`MetadataFeature`], if any.
    pub fn metadata(&self) -> Option<&MetadataFeature> {
        self.get(MetadataFeature::KIND).map(Feature::as_metadata)
    }

    /// Gets a reference to a [`TagFeature`], if any.
    pub fn tag(&self) -> Option<&TagFeature> {
        self.get(TagFeature::KIND).map(Feature::as_tag)
    }

    /// Gets a reference to a [`NativeTokenFeature`], if any.
    pub fn native_token(&self) -> Option<&NativeTokenFeature> {
        self.get(NativeTokenFeature::KIND).map(Feature::as_native_token)
    }

    /// Gets a reference to a [`BlockIssuerFeature`], if any.
    pub fn block_issuer(&self) -> Option<&BlockIssuerFeature> {
        self.get(BlockIssuerFeature::KIND).map(Feature::as_block_issuer)
    }

    /// Gets a reference to a [`StakingFeature`], if any.
    pub fn staking(&self) -> Option<&StakingFeature> {
        self.get(StakingFeature::KIND).map(Feature::as_staking)
    }
}

impl StorageScore for Features {
    fn storage_score(&self, params: StorageScoreParameters) -> u64 {
        self.iter().map(|f| f.storage_score(params)).sum::<u64>()
    }
}

#[inline]
fn verify_unique_sorted<const VERIFY: bool>(features: &[Feature]) -> Result<(), Error> {
    if VERIFY && !is_unique_sorted(features.iter().map(Feature::kind)) {
        Err(Error::FeaturesNotUniqueSorted)
    } else {
        Ok(())
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
    use pretty_assertions::assert_eq;

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
                FeatureFlags::NATIVE_TOKEN,
                FeatureFlags::BLOCK_ISSUER,
                FeatureFlags::STAKING
            ]
        );
    }
}
