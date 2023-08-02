// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod allotment;

/// The number of bits that a given mana value can use, excluding the sign bit.
pub const MANA_BITS: u64 = 63;
/// Equivalent to `2^MANA_BITS - 1`
pub const MAX_THEORETICAL_MANA: u64 = u64::MAX >> 1;

use core::ops::RangeInclusive;
use std::collections::HashSet;

use derive_more::Deref;
use packable::{bounded::BoundedU16, prefix::BoxedSlicePrefix, Packable};

pub use self::allotment::Allotment;
use super::{output::AccountId, Error};

pub(crate) type AllotmentCount = BoundedU16<{ *Allotments::COUNT_RANGE.start() }, { *Allotments::COUNT_RANGE.end() }>;

/// A list of [`Allotment`]s with unique [`AccountId`]s.
#[derive(Clone, Debug, Eq, PartialEq, Deref, Packable)]
#[packable(unpack_error = Error, with = |e| e.unwrap_item_err_or_else(|p| Error::InvalidAllotmentCount(p.into())))]
pub struct Allotments(#[packable(verify_with = verify_allotments)] BoxedSlicePrefix<Allotment, AllotmentCount>);

fn verify_allotments<const VERIFY: bool>(allotments: &[Allotment], _visitor: &()) -> Result<(), Error> {
    if VERIFY {
        let mut mana_sum: u64 = 0;
        let mut unique_ids = HashSet::with_capacity(allotments.len());
        for Allotment { account_id, mana } in allotments.iter() {
            mana_sum = mana_sum
                .checked_add(*mana)
                .ok_or(Error::InvalidAllotmentManaSum(mana_sum as u128 + *mana as u128))?;

            if mana_sum > MAX_THEORETICAL_MANA {
                return Err(Error::InvalidAllotmentManaSum(mana_sum as u128));
            }

            if !unique_ids.insert(account_id) {
                return Err(Error::DuplicateAllotment(*account_id));
            }
        }
    }

    Ok(())
}

impl TryFrom<Vec<Allotment>> for Allotments {
    type Error = Error;

    #[inline(always)]
    fn try_from(allotments: Vec<Allotment>) -> Result<Self, Self::Error> {
        Self::from_vec(allotments)
    }
}

impl IntoIterator for Allotments {
    type Item = Allotment;
    type IntoIter = alloc::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        Vec::from(Into::<Box<[Allotment]>>::into(self.0)).into_iter()
    }
}

impl Allotments {
    /// The minimum number of allotments of a transaction.
    pub const COUNT_MIN: u16 = 1;
    /// The maximum number of allotments of a transaction.
    pub const COUNT_MAX: u16 = 128;
    /// The range of valid numbers of allotments of a transaction.
    pub const COUNT_RANGE: RangeInclusive<u16> = Self::COUNT_MIN..=Self::COUNT_MAX; // [1..128]

    /// Creates a new [`Allotments`] from a vec.
    pub fn from_vec(allotments: Vec<Allotment>) -> Result<Self, Error> {
        let allotments = BoxedSlicePrefix::<Allotment, AllotmentCount>::try_from(allotments.into_boxed_slice())
            .map_err(Error::InvalidAllotmentCount)?;

        verify_allotments::<true>(&allotments, &())?;

        Ok(Self(allotments))
    }

    /// Gets a reference to an [`Allotment`], if one exists, using an [`AccountId`].
    #[inline(always)]
    pub fn get(&self, account_id: &AccountId) -> Option<&Allotment> {
        self.0
            .iter()
            .position(|a| a.account_id() == account_id)
            // PANIC: indexation is fine since the index has been found.
            .map(|index| &self.0[index])
    }
}
