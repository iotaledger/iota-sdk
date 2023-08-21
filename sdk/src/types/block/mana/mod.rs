// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod allotment;

use alloc::{boxed::Box, collections::BTreeSet, vec::Vec};
use core::ops::RangeInclusive;

use derive_more::Deref;
use iterator_sorted::is_unique_sorted;
use packable::{bounded::BoundedU16, prefix::BoxedSlicePrefix, Packable};

pub use self::allotment::ManaAllotment;
use super::{output::AccountId, Error};

/// The number of bits that a given mana value can use, excluding the sign bit.
pub const MANA_BITS: u64 = 63;
/// Equivalent to `2^MANA_BITS - 1`
pub const MAX_THEORETICAL_MANA: u64 = u64::MAX >> 1;

pub(crate) type ManaAllotmentCount =
    BoundedU16<{ *ManaAllotments::COUNT_RANGE.start() }, { *ManaAllotments::COUNT_RANGE.end() }>;

/// A list of [`ManaAllotment`]s with unique [`AccountId`]s.
#[derive(Clone, Debug, Eq, PartialEq, Deref, Packable)]
#[packable(unpack_error = Error, with = |e| e.unwrap_item_err_or_else(|p| Error::InvalidManaAllotmentCount(p.into())))]
pub struct ManaAllotments(
    #[packable(verify_with = verify_allotments)] BoxedSlicePrefix<ManaAllotment, ManaAllotmentCount>,
);

fn verify_allotments<const VERIFY: bool>(allotments: &[ManaAllotment], _visitor: &()) -> Result<(), Error> {
    if VERIFY {
        if !is_unique_sorted(allotments.iter().map(|a| a.account_id)) {
            return Err(Error::AllotmentsNotUniqueSorted);
        }
        verify_allotments_sum(allotments)?;
    }

    Ok(())
}

fn verify_allotments_sum<'a>(allotments: impl IntoIterator<Item = &'a ManaAllotment>) -> Result<(), Error> {
    let mut mana_sum: u64 = 0;

    for ManaAllotment { mana, .. } in allotments {
        mana_sum = mana_sum
            .checked_add(*mana)
            .ok_or(Error::InvalidManaAllotmentSum(mana_sum as u128 + *mana as u128))?;

        if mana_sum > MAX_THEORETICAL_MANA {
            return Err(Error::InvalidManaAllotmentSum(mana_sum as u128));
        }
    }

    Ok(())
}

impl TryFrom<Vec<ManaAllotment>> for ManaAllotments {
    type Error = Error;

    #[inline(always)]
    fn try_from(allotments: Vec<ManaAllotment>) -> Result<Self, Self::Error> {
        Self::from_vec(allotments)
    }
}

impl TryFrom<BTreeSet<ManaAllotment>> for ManaAllotments {
    type Error = Error;

    #[inline(always)]
    fn try_from(allotments: BTreeSet<ManaAllotment>) -> Result<Self, Self::Error> {
        Self::from_set(allotments)
    }
}

impl IntoIterator for ManaAllotments {
    type Item = ManaAllotment;
    type IntoIter = alloc::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        Vec::from(Into::<Box<[ManaAllotment]>>::into(self.0)).into_iter()
    }
}

impl ManaAllotments {
    /// The minimum number of allotments of a transaction.
    pub const COUNT_MIN: u16 = 1;
    /// The maximum number of allotments of a transaction.
    pub const COUNT_MAX: u16 = 128;
    /// The range of valid numbers of allotments of a transaction.
    pub const COUNT_RANGE: RangeInclusive<u16> = Self::COUNT_MIN..=Self::COUNT_MAX; // [1..128]

    /// Creates a new [`ManaAllotments`] from a vec.
    pub fn from_vec(allotments: Vec<ManaAllotment>) -> Result<Self, Error> {
        let allotments = BoxedSlicePrefix::<ManaAllotment, ManaAllotmentCount>::try_from(allotments.into_boxed_slice())
            .map_err(Error::InvalidManaAllotmentCount)?;

        verify_allotments::<true>(&allotments, &())?;

        Ok(Self(allotments))
    }

    /// Creates a new [`ManaAllotments`] from an ordered set.
    pub fn from_set(allotments: BTreeSet<ManaAllotment>) -> Result<Self, Error> {
        let allotments = BoxedSlicePrefix::<ManaAllotment, ManaAllotmentCount>::try_from(
            allotments.into_iter().collect::<Box<[_]>>(),
        )
        .map_err(Error::InvalidManaAllotmentCount)?;

        verify_allotments_sum(allotments.as_ref())?;

        Ok(Self(allotments))
    }

    /// Gets a reference to an [`ManaAllotment`], if one exists, using an [`AccountId`].
    #[inline(always)]
    pub fn get(&self, account_id: &AccountId) -> Option<&ManaAllotment> {
        self.0
            .iter()
            .position(|a| a.account_id() == account_id)
            // PANIC: indexation is fine since the index has been found.
            .map(|index| &self.0[index])
    }
}
