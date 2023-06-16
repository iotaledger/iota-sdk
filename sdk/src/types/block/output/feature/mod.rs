// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod issuer;
mod metadata;
mod sender;
mod tag;

use alloc::{boxed::Box, collections::BTreeSet, vec::Vec};

use bitflags::bitflags;
use derive_more::{Deref, From};
use iterator_sorted::is_unique_sorted;
use packable::{bounded::BoundedU8, prefix::BoxedSlicePrefix, Packable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize, Serializer};

pub use self::{issuer::IssuerFeature, metadata::MetadataFeature, sender::SenderFeature, tag::TagFeature};
pub(crate) use self::{metadata::MetadataFeatureLength, tag::TagFeatureLength};
use crate::{
    types::block::{create_bitflags, Error},
    utils::serde::boxed_slice_prefix,
};

///
#[derive(Clone, Eq, PartialEq, Hash, From, Packable)]
#[packable(unpack_error = Error)]
#[packable(tag_type = u8, with_error = Error::InvalidFeatureKind)]
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
}

impl PartialOrd for Feature {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.kind().partial_cmp(&other.kind())
    }
}
impl Ord for Feature {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl core::fmt::Debug for Feature {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Sender(feature) => feature.fmt(f),
            Self::Issuer(feature) => feature.fmt(f),
            Self::Metadata(feature) => feature.fmt(f),
            Self::Tag(feature) => feature.fmt(f),
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
        }
    }

    /// Returns the [`FeatureFlags`] for the given [`Feature`].
    pub fn flag(&self) -> FeatureFlags {
        match self {
            Self::Sender(_) => FeatureFlags::SENDER,
            Self::Issuer(_) => FeatureFlags::ISSUER,
            Self::Metadata(_) => FeatureFlags::METADATA,
            Self::Tag(_) => FeatureFlags::TAG,
        }
    }

    /// Checks whether the feature is a [`SenderFeature`].
    pub fn is_sender(&self) -> bool {
        matches!(self, Self::Sender(_))
    }

    /// Gets the feature as an actual [`SenderFeature`].
    /// PANIC: do not call on a non-sender feature.
    pub fn as_sender(&self) -> &SenderFeature {
        if let Self::Sender(feature) = self {
            feature
        } else {
            panic!("as_sender called on a non-sender feature");
        }
    }

    /// Checks whether the feature is an [`IssuerFeature`].
    pub fn is_issuer(&self) -> bool {
        matches!(self, Self::Issuer(_))
    }

    /// Gets the feature as an actual [`IssuerFeature`].
    /// PANIC: do not call on a non-issuer feature.
    pub fn as_issuer(&self) -> &IssuerFeature {
        if let Self::Issuer(feature) = self {
            feature
        } else {
            panic!("as_issuer called on a non-issuer feature");
        }
    }

    /// Checks whether the feature is a [`MetadataFeature`].
    pub fn is_metadata(&self) -> bool {
        matches!(self, Self::Metadata(_))
    }

    /// Gets the feature as an actual [`MetadataFeature`].
    /// PANIC: do not call on a non-metadata feature.
    pub fn as_metadata(&self) -> &MetadataFeature {
        if let Self::Metadata(feature) = self {
            feature
        } else {
            panic!("as_metadata called on a non-metadata feature");
        }
    }

    /// Checks whether the feature is a [`TagFeature`].
    pub fn is_tag(&self) -> bool {
        matches!(self, Self::Tag(_))
    }

    /// Gets the feature as an actual [`TagFeature`].
    /// PANIC: do not call on a non-tag feature.
    pub fn as_tag(&self) -> &TagFeature {
        if let Self::Tag(feature) = self {
            feature
        } else {
            panic!("as_tag called on a non-tag feature");
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Feature {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct TypedFeature {
            #[serde(rename = "type")]
            kind: u8,
            data: serde_json::Value,
        }

        let value = TypedFeature::deserialize(d)?;
        Ok(match value.kind {
            SenderFeature::KIND => SenderFeature::deserialize(value.data)
                .map_err(|e| serde::de::Error::custom(format!("cannot deserialize sender feature: {e}")))?
                .into(),
            IssuerFeature::KIND => IssuerFeature::deserialize(value.data)
                .map_err(|e| serde::de::Error::custom(format!("cannot deserialize issuer feature: {e}")))?
                .into(),
            MetadataFeature::KIND => MetadataFeature::deserialize(value.data)
                .map_err(|e| serde::de::Error::custom(format!("cannot deserialize metadata feature: {e}")))?
                .into(),
            TagFeature::KIND => TagFeature::deserialize(value.data)
                .map_err(|e| serde::de::Error::custom(format!("cannot deserialize tag feature: {e}")))?
                .into(),
            _ => {
                return Err(serde::de::Error::custom(format!(
                    "invalid feature type: {}",
                    value.kind
                )));
            }
        })
    }
}

#[cfg(feature = "serde")]
impl Serialize for Feature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        #[serde(untagged)]
        enum FeatureDto<'a> {
            T1(&'a SenderFeature),
            T2(&'a IssuerFeature),
            T3(&'a MetadataFeature),
            T4(&'a TagFeature),
        }
        #[derive(Serialize)]
        struct TypedFeature<'a> {
            #[serde(rename = "type")]
            kind: u8,
            data: FeatureDto<'a>,
        }
        let data = match self {
            Self::Sender(data) => FeatureDto::T1(data),
            Self::Issuer(data) => FeatureDto::T2(data),
            Self::Metadata(data) => FeatureDto::T3(data),
            Self::Tag(data) => FeatureDto::T4(data),
        };
        TypedFeature {
            kind: self.kind(),
            data,
        }
        .serialize(serializer)
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
    ]
);

pub(crate) type FeatureCount = BoundedU8<0, { Features::COUNT_MAX }>;

///
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Deref, Packable, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[packable(unpack_error = Error, with = |e| e.unwrap_item_err_or_else(|p| Error::InvalidFeatureCount(p.into())))]
pub struct Features(
    #[packable(verify_with = verify_unique_sorted)]
    #[serde(with = "boxed_slice_prefix")]
    BoxedSlicePrefix<Feature, FeatureCount>,
);

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
    ///
    pub const COUNT_MAX: u8 = 4;

    /// Creates a new [`Features`] from a vec.
    pub fn from_vec(features: Vec<Feature>) -> Result<Self, Error> {
        let mut features = BoxedSlicePrefix::<Feature, FeatureCount>::try_from(features.into_boxed_slice())
            .map_err(Error::InvalidFeatureCount)?;

        features.sort_by_key(Feature::kind);
        // Sort is obviously fine now but uniqueness still needs to be checked.
        verify_unique_sorted::<true>(&features, &())?;

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
        if let Some(Feature::Sender(sender)) = self.get(SenderFeature::KIND) {
            Some(sender)
        } else {
            None
        }
    }

    /// Gets a reference to a [`IssuerFeature`], if any.
    pub fn issuer(&self) -> Option<&IssuerFeature> {
        if let Some(Feature::Issuer(issuer)) = self.get(IssuerFeature::KIND) {
            Some(issuer)
        } else {
            None
        }
    }

    /// Gets a reference to a [`MetadataFeature`], if any.
    pub fn metadata(&self) -> Option<&MetadataFeature> {
        if let Some(Feature::Metadata(metadata)) = self.get(MetadataFeature::KIND) {
            Some(metadata)
        } else {
            None
        }
    }

    /// Gets a reference to a [`TagFeature`], if any.
    pub fn tag(&self) -> Option<&TagFeature> {
        if let Some(Feature::Tag(tag)) = self.get(TagFeature::KIND) {
            Some(tag)
        } else {
            None
        }
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[inline]
fn verify_unique_sorted<const VERIFY: bool>(features: &[Feature], _: &()) -> Result<(), Error> {
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
    use super::*;

    #[test]
    fn all_flags_present() {
        assert_eq!(
            FeatureFlags::ALL_FLAGS,
            &[
                FeatureFlags::SENDER,
                FeatureFlags::ISSUER,
                FeatureFlags::METADATA,
                FeatureFlags::TAG
            ]
        );
    }
}
