// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::{boxed::Box, collections::BTreeSet, vec::Vec};
use core::ops::RangeInclusive;

use crypto::signatures::ed25519;
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
    output::{Rent, RentBuilder},
    slot::SlotIndex,
    Error,
};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, From, packable::Packable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(untagged))]
#[packable(unpack_error = Error)]
#[packable(tag_type = u8, with_error = Error::InvalidBlockIssuerKeyKind)]
pub enum BlockIssuerKey {
    /// An Ed25519 block issuer key.
    #[packable(tag = Ed25519BlockIssuerKey::KIND)]
    Ed25519(Ed25519BlockIssuerKey),
}

impl core::fmt::Debug for BlockIssuerKey {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Ed25519(key) => key.fmt(f),
        }
    }
}

impl BlockIssuerKey {
    /// Returns the block issuer key kind of a [`BlockIssuerKey`].
    pub fn kind(&self) -> u8 {
        match self {
            Self::Ed25519(_) => Ed25519BlockIssuerKey::KIND,
        }
    }

    /// Checks whether the block issuer key is an [`Ed25519BlockIssuerKey`].
    pub fn is_ed25519(&self) -> bool {
        matches!(self, Self::Ed25519(_))
    }

    /// Gets the block issuer key as an actual [`Ed25519BlockIssuerKey`].
    /// NOTE: Will panic if the block issuer key is not a [`Ed25519BlockIssuerKey`].
    pub fn as_ed25519(&self) -> &Ed25519BlockIssuerKey {
        let Self::Ed25519(key) = self;
        key
    }
}

/// An Ed25519 block issuer key.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Deref, AsRef, From)]
#[as_ref(forward)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Ed25519BlockIssuerKey(ed25519::PublicKey);

impl Ed25519BlockIssuerKey {
    /// The block issuer key kind of an [`Ed25519BlockIssuerKey`].
    pub const KIND: u8 = 0;
    /// Length of an ED25519 block issuer key.
    pub const PUBLIC_KEY_LENGTH: usize = ed25519::PublicKey::LENGTH;

    /// Creates a new [`Ed25519BlockIssuerKey`] from bytes.
    pub fn try_from_bytes(bytes: [u8; Self::PUBLIC_KEY_LENGTH]) -> Result<Self, Error> {
        Ok(Self(ed25519::PublicKey::try_from_bytes(bytes)?))
    }
}

impl core::fmt::Debug for Ed25519BlockIssuerKey {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", prefix_hex::encode(self.0.as_slice()))
    }
}

impl Packable for Ed25519BlockIssuerKey {
    type UnpackError = Error;
    type UnpackVisitor = ();

    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        packer.pack_bytes(self.0.as_slice())?;
        Ok(())
    }

    fn unpack<U: Unpacker, const VERIFY: bool>(
        unpacker: &mut U,
        visitor: &Self::UnpackVisitor,
    ) -> Result<Self, UnpackError<Self::UnpackError, U::Error>> {
        Self::try_from_bytes(<[u8; Self::PUBLIC_KEY_LENGTH]>::unpack::<_, VERIFY>(unpacker, visitor).coerce()?)
            .map_err(UnpackError::Packable)
    }
}

pub(crate) type BlockIssuerKeyCount =
    BoundedU8<{ *BlockIssuerKeys::COUNT_RANGE.start() }, { *BlockIssuerKeys::COUNT_RANGE.end() }>;

/// Lexicographically ordered list of unique [`BlockIssuerKey`]
#[derive(Clone, Debug, Eq, PartialEq, Deref, Packable, Hash)]
#[packable(unpack_error = Error, with = |e| e.unwrap_item_err_or_else(|p| Error::InvalidBlockIssuerKeyCount(p.into())))]
pub struct BlockIssuerKeys(
    #[packable(verify_with = verify_block_issuer_keys)] BoxedSlicePrefix<BlockIssuerKey, BlockIssuerKeyCount>,
);

fn verify_block_issuer_keys<const VERIFY: bool>(
    block_issuer_keys: &[BlockIssuerKey],
    _visitor: &(),
) -> Result<(), Error> {
    if VERIFY && !is_unique_sorted(block_issuer_keys.iter()) {
        return Err(Error::BlockIssuerKeysNotUniqueSorted);
    }

    Ok(())
}

impl TryFrom<Vec<BlockIssuerKey>> for BlockIssuerKeys {
    type Error = Error;

    #[inline(always)]
    fn try_from(block_issuer_keys: Vec<BlockIssuerKey>) -> Result<Self, Self::Error> {
        Self::from_vec(block_issuer_keys)
    }
}

impl TryFrom<BTreeSet<BlockIssuerKey>> for BlockIssuerKeys {
    type Error = Error;

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
    /// The minimum number of block_issuer_keys in a [`BlockIssuerFeature`].
    pub const COUNT_MIN: u8 = 1;
    /// The maximum number of block_issuer_keys in a [`BlockIssuerFeature`].
    pub const COUNT_MAX: u8 = 128;
    /// The range of valid numbers of block_issuer_keys.
    pub const COUNT_RANGE: RangeInclusive<u8> = Self::COUNT_MIN..=Self::COUNT_MAX; // [1..128]

    /// Creates a new [`BlockIssuerKeys`] from a vec.
    pub fn from_vec(block_issuer_keys: Vec<BlockIssuerKey>) -> Result<Self, Error> {
        let mut block_issuer_keys =
            BoxedSlicePrefix::<BlockIssuerKey, BlockIssuerKeyCount>::try_from(block_issuer_keys.into_boxed_slice())
                .map_err(Error::InvalidBlockIssuerKeyCount)?;

        block_issuer_keys.sort();

        // Still need to verify the duplicate block issuer keys.
        verify_block_issuer_keys::<true>(&block_issuer_keys, &())?;

        Ok(Self(block_issuer_keys))
    }

    /// Creates a new [`BlockIssuerKeys`] from an ordered set.
    pub fn from_set(block_issuer_keys: BTreeSet<BlockIssuerKey>) -> Result<Self, Error> {
        let block_issuer_keys = BoxedSlicePrefix::<BlockIssuerKey, BlockIssuerKeyCount>::try_from(
            block_issuer_keys.into_iter().collect::<Box<[_]>>(),
        )
        .map_err(Error::InvalidBlockIssuerKeyCount)?;

        // We don't need to verify the block issuer keys here, because they are already verified by the BTreeSet.
        Ok(Self(block_issuer_keys))
    }
}

/// This feature defines the block issuer keys with which a signature from the containing
/// account's Block Issuance Credit can be verified in order to burn Mana.
#[derive(Clone, Debug, Eq, PartialEq, Hash, packable::Packable)]
#[packable(unpack_error = Error)]
pub struct BlockIssuerFeature {
    /// The slot index at which the Block Issuer Feature expires and can be removed.
    expiry_slot: SlotIndex,
    /// The Block Issuer Keys.
    block_issuer_keys: BlockIssuerKeys,
}

impl BlockIssuerFeature {
    /// The [`Feature`](crate::types::block::output::Feature) kind of a [`BlockIssuerFeature`].
    pub const KIND: u8 = 4;

    /// Creates a new [`BlockIssuerFeature`].
    #[inline(always)]
    pub fn new(
        expiry_slot: impl Into<SlotIndex>,
        block_issuer_keys: impl IntoIterator<Item = BlockIssuerKey>,
    ) -> Result<Self, Error> {
        let block_issuer_keys =
            BlockIssuerKeys::from_vec(block_issuer_keys.into_iter().collect::<Vec<BlockIssuerKey>>())?;
        Ok(Self {
            expiry_slot: expiry_slot.into(),
            block_issuer_keys,
        })
    }

    /// Returns the Slot Index at which the Block Issuer Feature expires and can be removed.
    pub fn expiry_slot(&self) -> SlotIndex {
        self.expiry_slot
    }

    /// Returns the Block Issuer Keys.
    pub fn block_issuer_keys(&self) -> &[BlockIssuerKey] {
        &self.block_issuer_keys
    }
}

impl Rent for BlockIssuerFeature {
    fn build_weighted_bytes(&self, builder: RentBuilder) -> RentBuilder {
        builder
            // Feature Type
            .data_field::<u8>()
            // Expiry Slot
            .data_field::<SlotIndex>()
            // Block Issuer Keys
            .packable_issuer_keys_field(&self.block_issuer_keys)
    }
}

#[cfg(feature = "serde")]
mod dto {
    use alloc::{string::String, vec::Vec};

    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::types::block::{slot::SlotIndex, Error};

    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, From)]
    #[serde(untagged)]
    pub enum BlockIssuerKeyDto {
        Ed25519(Ed25519BlockIssuerKeyDto),
    }

    impl From<&BlockIssuerKey> for BlockIssuerKeyDto {
        fn from(value: &BlockIssuerKey) -> Self {
            match value {
                BlockIssuerKey::Ed25519(s) => Self::Ed25519(s.into()),
            }
        }
    }

    impl TryFrom<BlockIssuerKeyDto> for BlockIssuerKey {
        type Error = Error;

        fn try_from(value: BlockIssuerKeyDto) -> Result<Self, Self::Error> {
            match value {
                BlockIssuerKeyDto::Ed25519(s) => Ok(Self::Ed25519(s.try_into()?)),
            }
        }
    }

    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Ed25519BlockIssuerKeyDto {
        #[serde(rename = "type")]
        pub kind: u8,
        pub public_key: String,
    }

    impl From<&Ed25519BlockIssuerKey> for Ed25519BlockIssuerKeyDto {
        fn from(value: &Ed25519BlockIssuerKey) -> Self {
            Self {
                kind: Ed25519BlockIssuerKey::KIND,
                public_key: prefix_hex::encode(value.0.as_slice()),
            }
        }
    }

    impl TryFrom<Ed25519BlockIssuerKeyDto> for Ed25519BlockIssuerKey {
        type Error = Error;

        fn try_from(value: Ed25519BlockIssuerKeyDto) -> Result<Self, Self::Error> {
            Self::try_from_bytes(prefix_hex::decode(value.public_key).map_err(|_| Error::InvalidField("publicKey"))?)
        }
    }

    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct BlockIssuerFeatureDto {
        #[serde(rename = "type")]
        kind: u8,
        expiry_slot: SlotIndex,
        block_issuer_keys: Vec<BlockIssuerKeyDto>,
    }

    impl From<&BlockIssuerFeature> for BlockIssuerFeatureDto {
        fn from(value: &BlockIssuerFeature) -> Self {
            Self {
                kind: BlockIssuerFeature::KIND,
                expiry_slot: value.expiry_slot,
                block_issuer_keys: value.block_issuer_keys.iter().map(|key| key.into()).collect(),
            }
        }
    }

    impl TryFrom<BlockIssuerFeatureDto> for BlockIssuerFeature {
        type Error = Error;

        fn try_from(value: BlockIssuerFeatureDto) -> Result<Self, Self::Error> {
            let keys = value
                .block_issuer_keys
                .into_iter()
                .map(BlockIssuerKey::try_from)
                .collect::<Result<Vec<BlockIssuerKey>, Error>>()?;

            Self::new(value.expiry_slot, keys)
        }
    }

    impl_serde_typed_dto!(BlockIssuerFeature, BlockIssuerFeatureDto, "block issuer feature");
}
