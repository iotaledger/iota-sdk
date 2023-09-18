// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod allotment;
mod protocol;

use alloc::{collections::BTreeSet, vec::Vec};
use core::ops::RangeInclusive;

use derive_more::Deref;
use packable::{
    bounded::BoundedU16,
    prefix::BTreeSetPrefix,
    set::{UnpackOrderedSetError, UnpackSetError},
    Packable,
};

#[cfg(feature = "serde")]
pub use self::allotment::dto::ManaAllotmentDto;
pub use self::{allotment::ManaAllotment, protocol::ManaStructure};
use super::{protocol::ProtocolParameters, Error};

pub(crate) type ManaAllotmentCount =
    BoundedU16<{ *ManaAllotments::COUNT_RANGE.start() }, { *ManaAllotments::COUNT_RANGE.end() }>;

/// A list of [`ManaAllotment`]s with unique [`AccountId`]s.
#[derive(Clone, Debug, Eq, PartialEq, Deref, Packable)]
#[packable(unpack_error = Error, with = map_mana_allotment_set_error)]
pub struct ManaAllotments(
    #[packable(verify_with = verify_mana_allotments)] BTreeSetPrefix<ManaAllotment, ManaAllotmentCount>,
);

fn map_mana_allotment_set_error<T, P>(error: UnpackOrderedSetError<T, Error, P>) -> Error
where
    <ManaAllotmentCount as TryFrom<usize>>::Error: From<P>,
{
    match error {
        UnpackOrderedSetError::Set(e) => match e {
            UnpackSetError::DuplicateItem(_) => Error::ManaAllotmentsNotUniqueSorted,
            UnpackSetError::Item(e) => e,
            UnpackSetError::Prefix(p) => Error::InvalidManaAllotmentCount(p.into()),
        },
        UnpackOrderedSetError::Unordered => Error::ManaAllotmentsNotUniqueSorted,
    }
}

impl ManaAllotments {
    /// The minimum number of mana allotments of a transaction.
    pub const COUNT_MIN: u16 = 0;
    /// The maximum number of mana allotments of a transaction.
    pub const COUNT_MAX: u16 = 128;
    /// The range of valid numbers of mana allotments of a transaction.
    pub const COUNT_RANGE: RangeInclusive<u16> = Self::COUNT_MIN..=Self::COUNT_MAX; // [1..128]

    /// Creates a new [`ManaAllotments`] from a vec.
    pub fn from_vec(allotments: Vec<ManaAllotment>) -> Result<Self, Error> {
        Ok(Self(
            allotments
                .into_iter()
                .collect::<BTreeSet<_>>()
                .try_into()
                .map_err(Error::InvalidManaAllotmentCount)?,
        ))
    }

    /// Creates a new [`ManaAllotments`] from an ordered set.
    pub fn from_set(allotments: BTreeSet<ManaAllotment>) -> Result<Self, Error> {
        Ok(Self(allotments.try_into().map_err(Error::InvalidManaAllotmentCount)?))
    }
}

fn verify_mana_allotments<const VERIFY: bool>(
    allotments: &BTreeSet<ManaAllotment>,
    protocol_params: &ProtocolParameters,
) -> Result<(), Error> {
    if VERIFY {
        verify_mana_allotments_sum(allotments, protocol_params)?;
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
    type IntoIter = alloc::collections::btree_set::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        BTreeSet::from(self.0).into_iter()
    }
}
