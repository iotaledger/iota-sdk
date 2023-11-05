// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::{collections::BTreeSet, vec::Vec};

use crate::types::block::{
    output::feature::{
        BlockIssuerFeature, BlockIssuerKey, BlockIssuerKeys, Ed25519BlockIssuerKey, Feature, FeatureFlags,
        GovernorMetadataFeature, IssuerFeature, MetadataFeature, SenderFeature, StakingFeature, TagFeature,
    },
    rand::{
        address::rand_address,
        bytes::rand_bytes,
        number::{rand_number, rand_number_range},
        slot::{rand_epoch_index, rand_slot_index},
    },
};

/// Generates a random [`SenderFeature`].
pub fn rand_sender_feature() -> SenderFeature {
    SenderFeature::new(rand_address())
}

/// Generates a random [`IssuerFeature`].
pub fn rand_issuer_feature() -> IssuerFeature {
    IssuerFeature::new(rand_address())
}

/// Generates a random [`MetadataFeature`].
pub fn rand_metadata_feature() -> MetadataFeature {
    let bytes = rand_bytes(rand_number_range(MetadataFeature::LENGTH_RANGE) as usize);
    MetadataFeature::new(bytes).unwrap()
}

/// Generates a random [`GovernorMetadataFeature`].
pub fn rand_governor_metadata_feature() -> GovernorMetadataFeature {
    let bytes = rand_bytes(rand_number_range(GovernorMetadataFeature::LENGTH_RANGE) as usize);
    GovernorMetadataFeature::new(bytes).unwrap()
}

/// Generates a random [`TagFeature`].
pub fn rand_tag_feature() -> TagFeature {
    let bytes = rand_bytes(rand_number_range(TagFeature::LENGTH_RANGE) as usize);
    TagFeature::new(bytes).unwrap()
}

/// Generates a valid random Ed25519 block issuer key.
pub fn rand_ed25519_block_issuer_key() -> Ed25519BlockIssuerKey {
    crypto::signatures::ed25519::SecretKey::generate()
        .unwrap()
        .public_key()
        .into()
}

/// Generates a valid random block issuer key.
pub fn rand_block_issuer_key() -> BlockIssuerKey {
    rand_ed25519_block_issuer_key().into()
}

/// Generates a vector of random valid block issuer keys of a given length.
pub fn rand_block_issuer_keys(len: usize) -> BTreeSet<BlockIssuerKey> {
    let mut block_issuer_keys: BTreeSet<BlockIssuerKey> = BTreeSet::new();

    while block_issuer_keys.len() < len {
        block_issuer_keys.insert(rand_block_issuer_key());
    }

    block_issuer_keys
}

/// Generates a random [`BlockIssuerFeature`].
pub fn rand_block_issuer_feature() -> BlockIssuerFeature {
    BlockIssuerFeature::new(
        rand_slot_index(),
        rand_block_issuer_keys(rand_number_range(
            BlockIssuerKeys::COUNT_MIN as usize..=BlockIssuerKeys::COUNT_MAX as usize,
        )),
    )
    .unwrap()
}

/// Generates a random [`StakingFeature`].
pub fn rand_staking_feature() -> StakingFeature {
    StakingFeature::new(rand_number(), rand_number(), rand_epoch_index(), rand_epoch_index())
}

fn rand_feature_from_flag(flag: &FeatureFlags) -> Feature {
    match *flag {
        FeatureFlags::SENDER => Feature::Sender(rand_sender_feature()),
        FeatureFlags::ISSUER => Feature::Issuer(rand_issuer_feature()),
        FeatureFlags::METADATA => Feature::Metadata(rand_metadata_feature()),
        FeatureFlags::GOVERNOR_METADATA => Feature::GovernorMetadata(rand_governor_metadata_feature()),
        FeatureFlags::TAG => Feature::Tag(rand_tag_feature()),
        FeatureFlags::BLOCK_ISSUER => Feature::BlockIssuer(rand_block_issuer_feature()),
        FeatureFlags::STAKING => Feature::Staking(rand_staking_feature()),
        _ => unreachable!(),
    }
}

/// Generates a [`Vec`] of random [`Feature`]s given a set of allowed [`FeatureFlags`].
pub fn rand_allowed_features(allowed_features: FeatureFlags) -> Vec<Feature> {
    let mut all_features = FeatureFlags::ALL_FLAGS
        .iter()
        .map(rand_feature_from_flag)
        .collect::<Vec<_>>();
    all_features.retain(|feature| allowed_features.contains(feature.flag()));
    all_features
}
