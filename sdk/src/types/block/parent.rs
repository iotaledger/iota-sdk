// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! The parents module defines the core data type for storing the blocks directly approved by a block.

use alloc::{boxed::Box, collections::BTreeSet, vec::Vec};
use core::ops::RangeInclusive;

use derive_more::Deref;
use iterator_sorted::is_unique_sorted;
use packable::{bounded::BoundedU8, prefix::BoxedSlicePrefix, Packable};

use crate::types::block::{BlockId, Error};

/// A [`Block`](crate::types::block::Block)'s [`Parents`] are the [`BlockId`]s of the blocks it directly approves.
///
/// Parents must be:
/// * in the `Parents::COUNT_RANGE` range;
/// * lexicographically sorted;
/// * unique;
#[derive(Clone, Debug, Eq, PartialEq, Deref, Packable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[deref(forward)]
#[packable(unpack_error = Error, with = |_| Error::InvalidParentCount)]
pub struct Parents<const MIN: u8, const MAX: u8>(
    #[packable(verify_with = verify_parents)] BoxedSlicePrefix<BlockId, BoundedU8<MIN, MAX>>,
);

impl<const MIN: u8, const MAX: u8> Parents<MIN, MAX> {
    /// The range representing the valid number of parents.
    pub const COUNT_RANGE: RangeInclusive<u8> = MIN..=MAX;

    /// Creates new [`Parents`] from a vec.
    pub fn from_vec(mut inner: Vec<BlockId>) -> Result<Self, Error> {
        inner.sort_unstable();
        inner.dedup();

        Ok(Self(
            inner
                .into_boxed_slice()
                .try_into()
                .map_err(|_| Error::InvalidParentCount)?,
        ))
    }

    /// Creates new [`Parents`] from an ordered set.
    pub fn from_set(inner: BTreeSet<BlockId>) -> Result<Self, Error> {
        Ok(Self(
            inner
                .into_iter()
                .collect::<Box<[_]>>()
                .try_into()
                .map_err(|_| Error::InvalidParentCount)?,
        ))
    }

    /// Returns the unique, ordered set of parents.
    pub fn to_set(&self) -> BTreeSet<BlockId> {
        self.0.iter().copied().collect()
    }

    /// Returns the number of parents.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns whether the parents list is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns an iterator over the parents.
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &BlockId> + '_ {
        self.0.iter()
    }
}

fn verify_parents<const VERIFY: bool>(parents: &[BlockId], _: &()) -> Result<(), Error> {
    if VERIFY && !is_unique_sorted(parents.iter().map(AsRef::as_ref)) {
        Err(Error::ParentsNotUniqueSorted)
    } else {
        Ok(())
    }
}

impl<const MAX: u8> Default for Parents<0, MAX> {
    fn default() -> Self {
        Self(Default::default())
    }
}

pub type StrongParents = Parents<1, 8>;
pub type WeakParents = Parents<0, 8>;
pub type ShallowLikeParents = Parents<0, 8>;
