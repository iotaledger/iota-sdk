// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::{boxed::Box, collections::BTreeSet, vec::Vec};
use core::ops::RangeInclusive;

use crypto::{
    hashes::{blake2b::Blake2b256, Digest},
    keys::bip44::Bip44,
    signatures::{ed25519, ed25519::PublicKey},
};
use derive_more::{AsRef, Deref, From};
use iterator_sorted::is_unique_sorted;
use packable::{
    bounded::BoundedU8,
    error::{UnpackError, UnpackErrorExt},
    packer::Packer,
    prefix::BoxedSlicePrefix,
    unpacker::Unpacker,
    Packable,
};

use crate::types::block::{
    output::{feature::FeatureError, StorageScore, StorageScoreParameters},
    protocol::{WorkScore, WorkScoreParameters},
    slot::SlotIndex,
};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, From, packable::Packable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(untagged))]
#[packable(unpack_error = FeatureError)]
#[packable(tag_type = u8, with_error = FeatureError::InvalidBlockIssuerKeyKind)]
pub enum BlockIssuerKey {
    /// An Ed25519 public key hash block issuer key.
    #[packable(tag = Ed25519PublicKeyHashBlockIssuerKey::KIND)]
    Ed25519PublicKeyHash(Ed25519PublicKeyHashBlockIssuerKey),
}

impl core::fmt::Debug for BlockIssuerKey {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Ed25519PublicKeyHash(key) => key.fmt(f),
        }
    }
}

impl BlockIssuerKey {
    /// Returns the block issuer key kind of a [`BlockIssuerKey`].
    pub fn kind(&self) -> u8 {
        match self {
            Self::Ed25519PublicKeyHash(_) => Ed25519PublicKeyHashBlockIssuerKey::KIND,
        }
    }

    crate::def_is_as_opt!(BlockIssuerKey: Ed25519PublicKeyHash);
}

impl StorageScore for BlockIssuerKey {
    fn storage_score(&self, params: StorageScoreParameters) -> u64 {
        match self {
            Self::Ed25519PublicKeyHash(e) => e.storage_score(params),
        }
    }
}

/// An Ed25519 public key hash block issuer key.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Deref, AsRef, From)]
#[as_ref(forward)]
pub struct Ed25519PublicKeyHashBlockIssuerKey([u8; Self::LENGTH]);

impl Ed25519PublicKeyHashBlockIssuerKey {
    /// The block issuer key kind of an [`Ed25519PublicKeyHashBlockIssuerKey`].
    pub const KIND: u8 = 0;
    /// Length of an ED25519 public key hash block issuer key.
    pub const LENGTH: usize = ed25519::PublicKey::LENGTH;

    /// Creates a new [`Ed25519PublicKeyHashBlockIssuerKey`] from bytes.
    pub fn new(bytes: [u8; Self::LENGTH]) -> Self {
        Self(bytes)
    }

    /// Creates a new [`Ed25519PublicKeyHashBlockIssuerKey`] from an [`ed25519::PublicKey`].
    pub fn from_public_key(public_key: ed25519::PublicKey) -> Self {
        Self(Blake2b256::digest(public_key.to_bytes()).into())
    }
}

impl StorageScore for Ed25519PublicKeyHashBlockIssuerKey {
    fn storage_score(&self, params: StorageScoreParameters) -> u64 {
        params.ed25519_block_issuer_key_offset()
    }
}

impl core::fmt::Debug for Ed25519PublicKeyHashBlockIssuerKey {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", prefix_hex::encode(self.0.as_slice()))
    }
}

impl Packable for Ed25519PublicKeyHashBlockIssuerKey {
    type UnpackError = FeatureError;
    type UnpackVisitor = ();

    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        packer.pack_bytes(self.0.as_slice())?;
        Ok(())
    }

    fn unpack<U: Unpacker>(
        unpacker: &mut U,
        visitor: Option<&Self::UnpackVisitor>,
    ) -> Result<Self, UnpackError<Self::UnpackError, U::Error>> {
        Ok(Self(<[u8; Self::LENGTH]>::unpack(unpacker, visitor).coerce()?))
    }
}

pub(crate) type BlockIssuerKeyCount =
    BoundedU8<{ *BlockIssuerKeys::COUNT_RANGE.start() }, { *BlockIssuerKeys::COUNT_RANGE.end() }>;

/// Lexicographically ordered list of unique [`BlockIssuerKey`]
#[derive(Clone, Debug, Eq, PartialEq, Deref, Packable, Hash)]
#[packable(unpack_error = FeatureError, with = |e| e.unwrap_item_err_or_else(|p| FeatureError::InvalidBlockIssuerKeyCount(p.into())))]
pub struct BlockIssuerKeys(
    #[packable(verify_with = verify_block_issuer_keys)] BoxedSlicePrefix<BlockIssuerKey, BlockIssuerKeyCount>,
);

fn verify_block_issuer_keys(block_issuer_keys: &[BlockIssuerKey]) -> Result<(), FeatureError> {
    if !is_unique_sorted(block_issuer_keys.iter()) {
        return Err(FeatureError::BlockIssuerKeysNotUniqueSorted);
    }

    Ok(())
}

impl TryFrom<Vec<BlockIssuerKey>> for BlockIssuerKeys {
    type Error = FeatureError;

    #[inline(always)]
    fn try_from(block_issuer_keys: Vec<BlockIssuerKey>) -> Result<Self, Self::Error> {
        Self::from_vec(block_issuer_keys)
    }
}

impl TryFrom<BTreeSet<BlockIssuerKey>> for BlockIssuerKeys {
    type Error = FeatureError;

    #[inline(always)]
    fn try_from(block_issuer_keys: BTreeSet<BlockIssuerKey>) -> Result<Self, Self::Error> {
        Self::from_set(block_issuer_keys)
    }
}

impl IntoIterator for BlockIssuerKeys {
    type Item = BlockIssuerKey;
    type IntoIter = alloc::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        Vec::from(Into::<Box<[BlockIssuerKey]>>::into(self.0)).into_iter()
    }
}

impl BlockIssuerKeys {
    /// The minimum number of block issuer keys in a [`BlockIssuerFeature`].
    pub const COUNT_MIN: u8 = 1;
    /// The maximum number of block issuer keys in a [`BlockIssuerFeature`].
    pub const COUNT_MAX: u8 = 128;
    /// The range of valid numbers of block issuer keys.
    pub const COUNT_RANGE: RangeInclusive<u8> = Self::COUNT_MIN..=Self::COUNT_MAX; // [1..128]

    /// Creates a new [`BlockIssuerKeys`] from a vec.
    pub fn from_vec(block_issuer_keys: Vec<BlockIssuerKey>) -> Result<Self, FeatureError> {
        let mut block_issuer_keys =
            BoxedSlicePrefix::<BlockIssuerKey, BlockIssuerKeyCount>::try_from(block_issuer_keys.into_boxed_slice())
                .map_err(FeatureError::InvalidBlockIssuerKeyCount)?;

        block_issuer_keys.sort();

        // Still need to verify the duplicate block issuer keys.
        verify_block_issuer_keys(&block_issuer_keys)?;

        Ok(Self(block_issuer_keys))
    }

    /// Creates a new [`BlockIssuerKeys`] from an ordered set.
    pub fn from_set(block_issuer_keys: BTreeSet<BlockIssuerKey>) -> Result<Self, FeatureError> {
        let block_issuer_keys = BoxedSlicePrefix::<BlockIssuerKey, BlockIssuerKeyCount>::try_from(
            block_issuer_keys.into_iter().collect::<Box<[_]>>(),
        )
        .map_err(FeatureError::InvalidBlockIssuerKeyCount)?;

        // We don't need to verify the block issuer keys here, because they are already verified by the BTreeSet.
        Ok(Self(block_issuer_keys))
    }
}

impl StorageScore for BlockIssuerKeys {
    fn storage_score(&self, params: StorageScoreParameters) -> u64 {
        self.iter().map(|b| b.storage_score(params)).sum::<u64>()
    }
}

/// This feature defines the block issuer keys with which a signature from the containing
/// account's Block Issuance Credit can be verified in order to burn Mana.
#[derive(Clone, Debug, Eq, PartialEq, Hash, packable::Packable)]
#[packable(unpack_error = FeatureError)]
pub struct BlockIssuerFeature {
    /// The slot index at which the feature expires and can be removed.
    expiry_slot: SlotIndex,
    /// The block issuer keys.
    block_issuer_keys: BlockIssuerKeys,
}

impl BlockIssuerFeature {
    /// The [`Feature`](crate::types::block::output::Feature) kind of a [`BlockIssuerFeature`].
    pub const KIND: u8 = 6;

    /// Creates a new [`BlockIssuerFeature`].
    #[inline(always)]
    pub fn new(
        expiry_slot: impl Into<SlotIndex>,
        block_issuer_keys: impl IntoIterator<Item = BlockIssuerKey>,
    ) -> Result<Self, FeatureError> {
        let block_issuer_keys =
            BlockIssuerKeys::from_vec(block_issuer_keys.into_iter().collect::<Vec<BlockIssuerKey>>())?;

        Ok(Self {
            expiry_slot: expiry_slot.into(),
            block_issuer_keys,
        })
    }

    /// Returns the expiry slot.
    pub fn expiry_slot(&self) -> SlotIndex {
        self.expiry_slot
    }

    /// Returns the block issuer keys.
    pub fn block_issuer_keys(&self) -> &[BlockIssuerKey] {
        &self.block_issuer_keys
    }
}

impl StorageScore for BlockIssuerFeature {
    fn storage_score(&self, params: StorageScoreParameters) -> u64 {
        self.block_issuer_keys.storage_score(params)
    }
}

impl WorkScore for BlockIssuerFeature {
    fn work_score(&self, params: WorkScoreParameters) -> u32 {
        params.block_issuer()
    }
}

#[derive(From)]
pub enum BlockIssuerKeySource {
    ImplicitAccountAddress,
    PublicKey(PublicKey),
    Bip44Path(Bip44),
}

#[cfg(feature = "serde")]
mod dto {
    use alloc::vec::Vec;

    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::{types::block::slot::SlotIndex, utils::serde::prefix_hex_bytes};

    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Ed25519PublicKeyHashBlockIssuerKeyDto {
        #[serde(rename = "type")]
        pub kind: u8,
        #[serde(with = "prefix_hex_bytes")]
        pub pub_key_hash: [u8; Ed25519PublicKeyHashBlockIssuerKey::LENGTH],
    }

    impl From<&Ed25519PublicKeyHashBlockIssuerKey> for Ed25519PublicKeyHashBlockIssuerKeyDto {
        fn from(value: &Ed25519PublicKeyHashBlockIssuerKey) -> Self {
            Self {
                kind: Ed25519PublicKeyHashBlockIssuerKey::KIND,
                pub_key_hash: value.0,
            }
        }
    }

    impl From<Ed25519PublicKeyHashBlockIssuerKeyDto> for Ed25519PublicKeyHashBlockIssuerKey {
        fn from(value: Ed25519PublicKeyHashBlockIssuerKeyDto) -> Self {
            Self(value.pub_key_hash)
        }
    }

    crate::impl_serde_typed_dto!(
        Ed25519PublicKeyHashBlockIssuerKey,
        Ed25519PublicKeyHashBlockIssuerKeyDto,
        "ed25519 public key hash block issuer key"
    );

    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct BlockIssuerFeatureDto {
        #[serde(rename = "type")]
        kind: u8,
        expiry_slot: SlotIndex,
        block_issuer_keys: Vec<BlockIssuerKey>,
    }

    impl From<&BlockIssuerFeature> for BlockIssuerFeatureDto {
        fn from(value: &BlockIssuerFeature) -> Self {
            Self {
                kind: BlockIssuerFeature::KIND,
                expiry_slot: value.expiry_slot,
                block_issuer_keys: value.block_issuer_keys.iter().cloned().collect(),
            }
        }
    }

    impl TryFrom<BlockIssuerFeatureDto> for BlockIssuerFeature {
        type Error = FeatureError;

        fn try_from(value: BlockIssuerFeatureDto) -> Result<Self, Self::Error> {
            Self::new(value.expiry_slot, BlockIssuerKeys::from_vec(value.block_issuer_keys)?)
        }
    }

    crate::impl_serde_typed_dto!(BlockIssuerFeature, BlockIssuerFeatureDto, "block issuer feature");

    crate::impl_deserialize_untagged!(BlockIssuerKey: Ed25519PublicKeyHash);
}
