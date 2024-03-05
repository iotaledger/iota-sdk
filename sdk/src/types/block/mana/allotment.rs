// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use alloc::{boxed::Box, collections::BTreeSet, vec::Vec};
use core::ops::RangeInclusive;

use derive_more::Deref;
use iterator_sorted::is_unique_sorted;
use packable::{bounded::BoundedU16, prefix::BoxedSlicePrefix, Packable};

use crate::types::block::{
    mana::ManaError,
    output::AccountId,
    protocol::{ProtocolParameters, WorkScore, WorkScoreParameters},
};

/// An allotment of Mana which will be added upon commitment of the slot in which the containing transaction was issued,
/// in the form of Block Issuance Credits to the account.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Packable)]
#[packable(unpack_error = ManaError)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "camelCase")
)]
pub struct ManaAllotment {
    pub(crate) account_id: AccountId,
    #[packable(verify_with = verify_mana)]
    #[cfg_attr(feature = "serde", serde(with = "crate::utils::serde::string"))]
    pub(crate) mana: u64,
}

impl ManaAllotment {
    pub fn new(account_id: AccountId, mana: u64) -> Result<Self, ManaError> {
        verify_mana(&mana)?;

        Ok(Self { account_id, mana })
    }

    pub fn account_id(&self) -> &AccountId {
        &self.account_id
    }

    pub fn mana(&self) -> u64 {
        self.mana
    }
}

impl PartialOrd for ManaAllotment {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ManaAllotment {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.account_id.cmp(&other.account_id)
    }
}

impl WorkScore for ManaAllotment {
    fn work_score(&self, params: WorkScoreParameters) -> u32 {
        params.allotment()
    }
}

fn verify_mana(mana: &u64) -> Result<(), ManaError> {
    if *mana == 0 {
        return Err(ManaError::Value(*mana));
    }

    Ok(())
}

pub(crate) type ManaAllotmentCount =
    BoundedU16<{ *ManaAllotments::COUNT_RANGE.start() }, { *ManaAllotments::COUNT_RANGE.end() }>;

/// A list of [`ManaAllotment`]s with unique [`AccountId`]s.
#[derive(Clone, Debug, Eq, PartialEq, Deref, Packable)]
#[packable(unpack_visitor = ProtocolParameters)]
#[packable(unpack_error = ManaError, with = |e| e.unwrap_item_err_or_else(|p| ManaError::AllotmentCount(p.into())))]
pub struct ManaAllotments(
    #[packable(verify_with = verify_mana_allotments)] BoxedSlicePrefix<ManaAllotment, ManaAllotmentCount>,
);

impl ManaAllotments {
    /// The minimum number of mana allotments of a transaction.
    pub const COUNT_MIN: u16 = 0;
    /// The maximum number of mana allotments of a transaction.
    pub const COUNT_MAX: u16 = 128;
    /// The range of valid numbers of mana allotments of a transaction.
    pub const COUNT_RANGE: RangeInclusive<u16> = Self::COUNT_MIN..=Self::COUNT_MAX; // [0..128]

    /// Creates a new [`ManaAllotments`] from a vec.
    pub fn from_vec(allotments: Vec<ManaAllotment>) -> Result<Self, ManaError> {
        verify_mana_allotments_unique_sorted(&allotments)?;

        Ok(Self(
            allotments
                .into_boxed_slice()
                .try_into()
                .map_err(ManaError::AllotmentCount)?,
        ))
    }

    /// Creates a new [`ManaAllotments`] from an ordered set.
    pub fn from_set(allotments: BTreeSet<ManaAllotment>) -> Result<Self, ManaError> {
        Ok(Self(
            allotments
                .into_iter()
                .collect::<Box<[_]>>()
                .try_into()
                .map_err(ManaError::AllotmentCount)?,
        ))
    }

    /// Gets a reference to an [`ManaAllotment`], if one exists, using an [`AccountId`].
    #[inline(always)]
    pub fn get(&self, account_id: &AccountId) -> Option<&ManaAllotment> {
        self.0.iter().find(|a| a.account_id() == account_id)
    }
}

fn verify_mana_allotments(allotments: &[ManaAllotment], protocol_params: &ProtocolParameters) -> Result<(), ManaError> {
    verify_mana_allotments_unique_sorted(allotments)?;
    verify_mana_allotments_sum(allotments, protocol_params)?;

    Ok(())
}

fn verify_mana_allotments_unique_sorted<'a>(
    allotments: impl IntoIterator<Item = &'a ManaAllotment>,
) -> Result<(), ManaError> {
    if !is_unique_sorted(allotments.into_iter()) {
        return Err(ManaError::AllotmentsNotUniqueSorted);
    }
    Ok(())
}

pub(crate) fn verify_mana_allotments_sum<'a>(
    allotments: impl IntoIterator<Item = &'a ManaAllotment>,
    protocol_params: &ProtocolParameters,
) -> Result<(), ManaError> {
    let mut mana_sum: u64 = 0;
    let max_mana = protocol_params.mana_parameters().max_mana();

    for ManaAllotment { mana, .. } in allotments {
        mana_sum = mana_sum.checked_add(*mana).ok_or(ManaError::AllotmentSum {
            sum: mana_sum as u128 + *mana as u128,
            max: max_mana,
        })?;

        if mana_sum > max_mana {
            return Err(ManaError::AllotmentSum {
                sum: mana_sum as u128,
                max: max_mana,
            });
        }
    }

    Ok(())
}

impl TryFrom<Vec<ManaAllotment>> for ManaAllotments {
    type Error = ManaError;

    #[inline(always)]
    fn try_from(allotments: Vec<ManaAllotment>) -> Result<Self, Self::Error> {
        Self::from_vec(allotments)
    }
}

impl TryFrom<BTreeSet<ManaAllotment>> for ManaAllotments {
    type Error = ManaError;

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
