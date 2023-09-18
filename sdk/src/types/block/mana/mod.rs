// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod allotment;
mod protocol;

use alloc::{boxed::Box, collections::BTreeSet, vec::Vec};
use core::ops::RangeInclusive;

use derive_more::Deref;
use iterator_sorted::is_unique_sorted;
use packable::{bounded::BoundedU16, prefix::BoxedSlicePrefix, Packable};

#[cfg(feature = "serde")]
pub use self::allotment::dto::ManaAllotmentDto;
pub use self::{allotment::ManaAllotment, protocol::ManaStructure};
use super::{output::AccountId, protocol::ProtocolParameters, Error};

pub(crate) type ManaAllotmentCount =
    BoundedU16<{ *ManaAllotments::COUNT_RANGE.start() }, { *ManaAllotments::COUNT_RANGE.end() }>;

/// A list of [`ManaAllotment`]s with unique [`AccountId`]s.
#[derive(Clone, Debug, Eq, PartialEq, Deref, Packable)]
#[packable(unpack_error = Error, with = |e| e.unwrap_item_err_or_else(|p| Error::InvalidManaAllotmentCount(p.into())))]
pub struct ManaAllotments(
    #[packable(verify_with = verify_mana_allotments)] BoxedSlicePrefix<ManaAllotment, ManaAllotmentCount>,
);

impl ManaAllotments {
    /// The minimum number of mana allotments of a transaction.
    pub const COUNT_MIN: u16 = 0;
    /// The maximum number of mana allotments of a transaction.
    pub const COUNT_MAX: u16 = 128;
    /// The range of valid numbers of mana allotments of a transaction.
    pub const COUNT_RANGE: RangeInclusive<u16> = Self::COUNT_MIN..=Self::COUNT_MAX; // [1..128]

    /// Creates a new [`ManaAllotments`] from a vec.
    pub fn from_vec(allotments: Vec<ManaAllotment>) -> Result<Self, Error> {
        verify_mana_allotments_unique_sorted(&allotments)?;

        Ok(Self(
            allotments
                .into_boxed_slice()
                .try_into()
                .map_err(Error::InvalidManaAllotmentCount)?,
        ))
    }

    /// Creates a new [`ManaAllotments`] from an ordered set.
    pub fn from_set(allotments: BTreeSet<ManaAllotment>) -> Result<Self, Error> {
        Ok(Self(
            allotments
                .into_iter()
                .collect::<Box<[_]>>()
                .try_into()
                .map_err(Error::InvalidManaAllotmentCount)?,
        ))
    }

    /// Gets a reference to an [`ManaAllotment`], if one exists, using an [`AccountId`].
    #[inline(always)]
    pub fn get(&self, account_id: &AccountId) -> Option<&ManaAllotment> {
        self.0.iter().find(|a| a.account_id() == account_id)
    }
}

fn verify_mana_allotments<const VERIFY: bool>(
    allotments: &[ManaAllotment],
    protocol_params: &ProtocolParameters,
) -> Result<(), Error> {
    if VERIFY {
        verify_mana_allotments_unique_sorted(allotments)?;
        verify_mana_allotments_sum(allotments, protocol_params)?;
    }

    Ok(())
}

fn verify_mana_allotments_unique_sorted<'a>(
    allotments: impl IntoIterator<Item = &'a ManaAllotment>,
) -> Result<(), Error> {
    if !is_unique_sorted(allotments.into_iter()) {
        return Err(Error::ManaAllotmentsNotUniqueSorted);
    }
    Ok(())
}

pub(crate) fn verify_mana_allotments_sum<'a>(
    allotments: impl IntoIterator<Item = &'a ManaAllotment>,
    protocol_params: &ProtocolParameters,
) -> Result<(), Error> {
    let mut mana_sum: u64 = 0;
    let max_mana = protocol_params.mana_structure().max_mana();

    for ManaAllotment { mana, .. } in allotments {
        mana_sum = mana_sum.checked_add(*mana).ok_or(Error::InvalidManaAllotmentSum {
            sum: mana_sum as u128 + *mana as u128,
            max: max_mana,
        })?;

        if mana_sum > max_mana {
            return Err(Error::InvalidManaAllotmentSum {
                sum: mana_sum as u128,
                max: max_mana,
            });
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
