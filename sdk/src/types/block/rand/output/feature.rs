// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::{
    collections::{BTreeMap, BTreeSet},
    vec::Vec,
};
use core::ops::Range;

use rand::distributions::{Alphanumeric, DistString};

use crate::types::block::{
    output::feature::{
        BlockIssuerFeature, BlockIssuerKey, BlockIssuerKeys, Ed25519BlockIssuerKey, Feature, FeatureFlags,
        IssuerFeature, MetadataFeature, NativeTokenFeature, SenderFeature, StakingFeature, TagFeature,
    },
    rand::{
        address::rand_address,
        bytes::rand_bytes,
        number::{rand_number, rand_number_range},
        output::rand_native_token,
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
    let mut map = BTreeMap::new();
    // Starting at 1 for entries count bytes
    let mut total_size = 1;
    let max_size = *MetadataFeature::BYTE_LENGTH_RANGE.end() as usize;
    let key_prefix_length = 1;
    let value_prefix_length = 2;

    for _ in 0..10 {
        // +1 since min key size is 1
        if total_size > (max_size - (key_prefix_length + value_prefix_length + 1)) as usize {
            break;
        }

        // Key length
        total_size += key_prefix_length;
        let max_val = if max_size - total_size - value_prefix_length < u8::MAX as usize {
            max_size - total_size - value_prefix_length
        } else {
            u8::MAX.into()
        };

        let key = if max_val == 1 {
            "a".to_string()
        } else {
            Alphanumeric.sample_string(
                &mut rand::thread_rng(),
                rand_number_range(Range { start: 1, end: max_val }),
            )
        };
        total_size += key.as_bytes().len();

        if total_size > max_size - value_prefix_length {
            // println!("breaking before adding more");
            break;
        }

        // Value length
        total_size += value_prefix_length;
        let bytes = if max_size - total_size == 0 {
            vec![]
        } else {
            rand_bytes(rand_number_range(Range {
                start: 0,
                end: max_size - total_size,
            }))
        };
        total_size += bytes.len();

        map.insert(key.into(), bytes);
    }

    MetadataFeature::new(map).unwrap()
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

/// Generates a random [`NativeTokenFeature`].
pub fn rand_native_token_feature() -> NativeTokenFeature {
    NativeTokenFeature::new(rand_native_token())
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
        FeatureFlags::TAG => Feature::Tag(rand_tag_feature()),
        FeatureFlags::NATIVE_TOKEN => Feature::NativeToken(rand_native_token_feature()),
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
