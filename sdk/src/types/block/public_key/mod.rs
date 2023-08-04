// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod ed25519;

use alloc::{boxed::Box, collections::BTreeSet, vec::Vec};
use core::ops::RangeInclusive;

use derive_more::{Deref, From};
use iterator_sorted::is_unique_sorted;
use packable::{bounded::BoundedU8, prefix::BoxedSlicePrefix, Packable};

pub use self::ed25519::Ed25519PublicKey;
use crate::types::block::Error;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, From, packable::Packable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(untagged))]
#[packable(unpack_error = Error)]
#[packable(tag_type = u8, with_error = Error::InvalidPublicKeyKind)]
pub enum PublicKey {
    /// An Ed25519 public key.
    #[packable(tag = Ed25519PublicKey::KIND)]
    Ed25519(Ed25519PublicKey),
}

impl core::fmt::Debug for PublicKey {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Ed25519(public_key) => public_key.fmt(f),
        }
    }
}

impl PublicKey {
    /// Returns the public key kind of a [`PublicKey`].
    pub fn kind(&self) -> u8 {
        match self {
            Self::Ed25519(_) => Ed25519PublicKey::KIND,
        }
    }

    /// Checks whether the public key is an [`Ed25519PublicKey`].
    pub fn is_ed25519(&self) -> bool {
        matches!(self, Self::Ed25519(_))
    }

    /// Gets the public key as an actual [`Ed25519PublicKey`].
    /// NOTE: Will panic if the public key is not a [`Ed25519PublicKey`].
    pub fn as_ed25519(&self) -> &Ed25519PublicKey {
        let Self::Ed25519(public_key) = self;
        public_key
    }
}

pub(crate) mod dto {
    use serde::{Deserialize, Serialize};

    use super::{ed25519::dto::Ed25519PublicKeyDto, *};
    use crate::types::block::Error;

    /// Describes all the different public key types.
    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, From)]
    #[serde(untagged)]
    pub enum PublicKeyDto {
        Ed25519(Ed25519PublicKeyDto),
    }

    impl From<&PublicKey> for PublicKeyDto {
        fn from(value: &PublicKey) -> Self {
            match value {
                PublicKey::Ed25519(s) => Self::Ed25519(s.into()),
            }
        }
    }

    impl TryFrom<PublicKeyDto> for PublicKey {
        type Error = Error;

        fn try_from(value: PublicKeyDto) -> Result<Self, Self::Error> {
            match value {
                PublicKeyDto::Ed25519(s) => Ok(Self::Ed25519(s.try_into()?)),
            }
        }
    }
}

pub(crate) type PublicKeyCount = BoundedU8<{ *PublicKeys::COUNT_RANGE.start() }, { *PublicKeys::COUNT_RANGE.end() }>;

/// Lexicographically ordered list of unique [`PublicKey`]
#[derive(Clone, Debug, Eq, PartialEq, Deref, Packable, Hash)]
#[packable(unpack_error = Error, with = |e| e.unwrap_item_err_or_else(|p| Error::InvalidPublicKeyCount(p.into())))]
pub struct PublicKeys(#[packable(verify_with = verify_public_keys)] BoxedSlicePrefix<PublicKey, PublicKeyCount>);

fn verify_public_keys<const VERIFY: bool>(public_keys: &[PublicKey], _visitor: &()) -> Result<(), Error> {
    if VERIFY && !is_unique_sorted(public_keys.iter()) {
        return Err(Error::PublicKeysNotUniqueSorted);
    }

    Ok(())
}

impl TryFrom<Vec<PublicKey>> for PublicKeys {
    type Error = Error;

    #[inline(always)]
    fn try_from(public_keys: Vec<PublicKey>) -> Result<Self, Self::Error> {
        Self::from_vec(public_keys)
    }
}

impl TryFrom<BTreeSet<PublicKey>> for PublicKeys {
    type Error = Error;

    #[inline(always)]
    fn try_from(public_keys: BTreeSet<PublicKey>) -> Result<Self, Self::Error> {
        Self::from_set(public_keys)
    }
}

impl IntoIterator for PublicKeys {
    type Item = PublicKey;
    type IntoIter = alloc::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        Vec::from(Into::<Box<[PublicKey]>>::into(self.0)).into_iter()
    }
}

impl PublicKeys {
    /// The minimum number of public_keys of a transaction.
    pub const COUNT_MIN: u8 = 1;
    /// The maximum number of public_keys of a transaction.
    pub const COUNT_MAX: u8 = 128;
    /// The range of valid numbers of public_keys.
    pub const COUNT_RANGE: RangeInclusive<u8> = Self::COUNT_MIN..=Self::COUNT_MAX; // [1..128]

    /// Creates a new [`PublicKeys`] from a vec.
    pub fn from_vec(public_keys: Vec<PublicKey>) -> Result<Self, Error> {
        let mut public_keys = BoxedSlicePrefix::<PublicKey, PublicKeyCount>::try_from(public_keys.into_boxed_slice())
            .map_err(Error::InvalidPublicKeyCount)?;

        public_keys.sort();

        // Still need to verify the duplicate public keys.
        verify_public_keys::<true>(&public_keys, &())?;

        Ok(Self(public_keys))
    }

    /// Creates a new [`PublicKeys`] from an ordered set.
    pub fn from_set(public_keys: BTreeSet<PublicKey>) -> Result<Self, Error> {
        let public_keys =
            BoxedSlicePrefix::<PublicKey, PublicKeyCount>::try_from(public_keys.into_iter().collect::<Box<[_]>>())
                .map_err(Error::InvalidPublicKeyCount)?;

        // We don't need to verify the public keys here, because they are already verified by the BTreeSet.
        Ok(Self(public_keys))
    }
}
