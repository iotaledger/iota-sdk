// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod address;
mod expiration;
mod governor_address;
mod immutable_account_address;
mod state_controller_address;
mod storage_deposit_return;
mod timelock;

use alloc::{boxed::Box, collections::BTreeSet, vec::Vec};

use bitflags::bitflags;
use derive_more::{Deref, From};
use iterator_sorted::is_unique_sorted;
use packable::{bounded::BoundedU8, prefix::BoxedSlicePrefix, Packable};

pub use self::{
    address::AddressUnlockCondition, expiration::ExpirationUnlockCondition,
    governor_address::GovernorAddressUnlockCondition,
    immutable_account_address::ImmutableAccountAddressUnlockCondition,
    state_controller_address::StateControllerAddressUnlockCondition,
    storage_deposit_return::StorageDepositReturnUnlockCondition, timelock::TimelockUnlockCondition,
};
use crate::types::block::{
    address::Address,
    output::{StorageScore, StorageScoreParameters},
    protocol::{ProtocolParameters, WorkScore},
    slot::SlotIndex,
    Error,
};

///
#[derive(Clone, Eq, PartialEq, Hash, From, Packable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(untagged))]
#[packable(unpack_error = Error)]
#[packable(unpack_visitor = ProtocolParameters)]
#[packable(tag_type = u8, with_error = Error::InvalidUnlockConditionKind)]
pub enum UnlockCondition {
    /// An address unlock condition.
    #[packable(tag = AddressUnlockCondition::KIND)]
    Address(AddressUnlockCondition),
    /// A storage deposit return unlock condition.
    #[packable(tag = StorageDepositReturnUnlockCondition::KIND)]
    StorageDepositReturn(StorageDepositReturnUnlockCondition),
    /// A timelock unlock condition.
    #[packable(tag = TimelockUnlockCondition::KIND)]
    Timelock(TimelockUnlockCondition),
    /// An expiration unlock condition.
    #[packable(tag = ExpirationUnlockCondition::KIND)]
    Expiration(ExpirationUnlockCondition),
    /// A state controller address unlock condition.
    #[packable(tag = StateControllerAddressUnlockCondition::KIND)]
    StateControllerAddress(StateControllerAddressUnlockCondition),
    /// A governor address unlock condition.
    #[packable(tag = GovernorAddressUnlockCondition::KIND)]
    GovernorAddress(GovernorAddressUnlockCondition),
    /// An immutable account address unlock condition.
    #[packable(tag = ImmutableAccountAddressUnlockCondition::KIND)]
    ImmutableAccountAddress(ImmutableAccountAddressUnlockCondition),
}

impl PartialOrd for UnlockCondition {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for UnlockCondition {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.kind().cmp(&other.kind())
    }
}

impl StorageScore for UnlockCondition {
    fn storage_score(&self, params: StorageScoreParameters) -> u64 {
        match self {
            Self::Address(uc) => uc.storage_score(params),
            Self::StorageDepositReturn(uc) => uc.storage_score(params),
            Self::Timelock(uc) => uc.storage_score(params),
            Self::Expiration(uc) => uc.storage_score(params),
            Self::StateControllerAddress(uc) => uc.storage_score(params),
            Self::GovernorAddress(uc) => uc.storage_score(params),
            Self::ImmutableAccountAddress(uc) => uc.storage_score(params),
        }
    }
}

// TODO: check with TIP
impl WorkScore for UnlockCondition {}

impl core::fmt::Debug for UnlockCondition {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Address(uc) => uc.fmt(f),
            Self::StorageDepositReturn(uc) => uc.fmt(f),
            Self::Timelock(uc) => uc.fmt(f),
            Self::Expiration(uc) => uc.fmt(f),
            Self::StateControllerAddress(uc) => uc.fmt(f),
            Self::GovernorAddress(uc) => uc.fmt(f),
            Self::ImmutableAccountAddress(uc) => uc.fmt(f),
        }
    }
}

impl UnlockCondition {
    /// Return the output kind of an `Output`.
    pub fn kind(&self) -> u8 {
        match self {
            Self::Address(_) => AddressUnlockCondition::KIND,
            Self::StorageDepositReturn(_) => StorageDepositReturnUnlockCondition::KIND,
            Self::Timelock(_) => TimelockUnlockCondition::KIND,
            Self::Expiration(_) => ExpirationUnlockCondition::KIND,
            Self::StateControllerAddress(_) => StateControllerAddressUnlockCondition::KIND,
            Self::GovernorAddress(_) => GovernorAddressUnlockCondition::KIND,
            Self::ImmutableAccountAddress(_) => ImmutableAccountAddressUnlockCondition::KIND,
        }
    }

    /// Returns the [`UnlockConditionFlags`] for the given [`UnlockCondition`].
    pub(crate) fn flag(&self) -> UnlockConditionFlags {
        match self {
            Self::Address(_) => UnlockConditionFlags::ADDRESS,
            Self::StorageDepositReturn(_) => UnlockConditionFlags::STORAGE_DEPOSIT_RETURN,
            Self::Timelock(_) => UnlockConditionFlags::TIMELOCK,
            Self::Expiration(_) => UnlockConditionFlags::EXPIRATION,
            Self::StateControllerAddress(_) => UnlockConditionFlags::STATE_CONTROLLER_ADDRESS,
            Self::GovernorAddress(_) => UnlockConditionFlags::GOVERNOR_ADDRESS,
            Self::ImmutableAccountAddress(_) => UnlockConditionFlags::IMMUTABLE_ACCOUNT_ADDRESS,
        }
    }

    crate::def_is_as_opt!(UnlockCondition:
        Address,
        StorageDepositReturn,
        Timelock,
        Expiration,
        StateControllerAddress,
        GovernorAddress,
        ImmutableAccountAddress
    );
}

crate::create_bitflags!(
    /// A bitflags-based representation of the set of active [`UnlockCondition`]s.
    pub UnlockConditionFlags,
    u16,
    [
        (ADDRESS, AddressUnlockCondition),
        (STORAGE_DEPOSIT_RETURN, StorageDepositReturnUnlockCondition),
        (TIMELOCK, TimelockUnlockCondition),
        (EXPIRATION, ExpirationUnlockCondition),
        (STATE_CONTROLLER_ADDRESS, StateControllerAddressUnlockCondition),
        (GOVERNOR_ADDRESS, GovernorAddressUnlockCondition),
        (IMMUTABLE_ACCOUNT_ADDRESS, ImmutableAccountAddressUnlockCondition),
    ]
);

pub(crate) type UnlockConditionCount = BoundedU8<0, { UnlockConditionFlags::ALL_FLAGS.len() as u8 }>;

///
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Deref, Packable)]
#[packable(unpack_error = Error, with = |e| e.unwrap_item_err_or_else(|p| Error::InvalidUnlockConditionCount(p.into())))]
#[packable(unpack_visitor = ProtocolParameters)]
pub struct UnlockConditions(
    #[packable(verify_with = verify_unique_sorted_packable)] BoxedSlicePrefix<UnlockCondition, UnlockConditionCount>,
);

impl TryFrom<Vec<UnlockCondition>> for UnlockConditions {
    type Error = Error;

    #[inline(always)]
    fn try_from(unlock_conditions: Vec<UnlockCondition>) -> Result<Self, Self::Error> {
        Self::from_vec(unlock_conditions)
    }
}

impl TryFrom<BTreeSet<UnlockCondition>> for UnlockConditions {
    type Error = Error;

    #[inline(always)]
    fn try_from(unlock_conditions: BTreeSet<UnlockCondition>) -> Result<Self, Self::Error> {
        Self::from_set(unlock_conditions)
    }
}

impl IntoIterator for UnlockConditions {
    type Item = UnlockCondition;
    type IntoIter = alloc::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        Vec::from(Into::<Box<[UnlockCondition]>>::into(self.0)).into_iter()
    }
}

impl UnlockConditions {
    /// Creates a new [`UnlockConditions`] from a vec.
    pub fn from_vec(unlock_conditions: Vec<UnlockCondition>) -> Result<Self, Error> {
        let mut unlock_conditions =
            BoxedSlicePrefix::<UnlockCondition, UnlockConditionCount>::try_from(unlock_conditions.into_boxed_slice())
                .map_err(Error::InvalidUnlockConditionCount)?;

        unlock_conditions.sort_by_key(UnlockCondition::kind);
        // Sort is obviously fine now but uniqueness still needs to be checked.
        verify_unique_sorted::<true>(&unlock_conditions)?;

        Ok(Self(unlock_conditions))
    }

    /// Creates a new [`UnlockConditions`] from an ordered set.
    pub fn from_set(unlock_conditions: BTreeSet<UnlockCondition>) -> Result<Self, Error> {
        Ok(Self(
            unlock_conditions
                .into_iter()
                .collect::<Box<[_]>>()
                .try_into()
                .map_err(Error::InvalidUnlockConditionCount)?,
        ))
    }

    /// Gets a reference to an [`UnlockCondition`] from an unlock condition kind, if any.
    #[inline(always)]
    pub fn get(&self, key: u8) -> Option<&UnlockCondition> {
        self.0
            .binary_search_by_key(&key, UnlockCondition::kind)
            // PANIC: indexation is fine since the index has been found.
            .map(|index| &self.0[index])
            .ok()
    }

    /// Gets a reference to an [`AddressUnlockCondition`], if any.
    #[inline(always)]
    pub fn address(&self) -> Option<&AddressUnlockCondition> {
        self.get(AddressUnlockCondition::KIND).map(UnlockCondition::as_address)
    }

    /// Gets a reference to a [`StorageDepositReturnUnlockCondition`], if any.
    #[inline(always)]
    pub fn storage_deposit_return(&self) -> Option<&StorageDepositReturnUnlockCondition> {
        self.get(StorageDepositReturnUnlockCondition::KIND)
            .map(UnlockCondition::as_storage_deposit_return)
    }

    /// Gets a reference to a [`TimelockUnlockCondition`], if any.
    #[inline(always)]
    pub fn timelock(&self) -> Option<&TimelockUnlockCondition> {
        self.get(TimelockUnlockCondition::KIND)
            .map(UnlockCondition::as_timelock)
    }

    /// Checks whether a timelock exists and is still relevant.
    #[inline(always)]
    pub fn is_timelocked(&self, slot_index: impl Into<SlotIndex>, min_committable_age: impl Into<SlotIndex>) -> bool {
        self.timelock().map_or(false, |timelock| {
            timelock.is_timelocked(slot_index, min_committable_age)
        })
    }

    /// Gets a reference to an [`ExpirationUnlockCondition`], if any.
    #[inline(always)]
    pub fn expiration(&self) -> Option<&ExpirationUnlockCondition> {
        self.get(ExpirationUnlockCondition::KIND)
            .map(UnlockCondition::as_expiration)
    }

    /// Returns whether an expiration exists and is expired.
    #[inline(always)]
    pub fn is_expired(&self, slot_index: impl Into<SlotIndex>) -> bool {
        let slot_index = slot_index.into();

        self.expiration()
            .map_or(false, |expiration| slot_index >= expiration.slot_index())
    }

    /// Gets a reference to a [`StateControllerAddressUnlockCondition`], if any.
    #[inline(always)]
    pub fn state_controller_address(&self) -> Option<&StateControllerAddressUnlockCondition> {
        self.get(StateControllerAddressUnlockCondition::KIND)
            .map(UnlockCondition::as_state_controller_address)
    }

    /// Gets a reference to a [`GovernorAddressUnlockCondition`], if any.
    #[inline(always)]
    pub fn governor_address(&self) -> Option<&GovernorAddressUnlockCondition> {
        self.get(GovernorAddressUnlockCondition::KIND)
            .map(UnlockCondition::as_governor_address)
    }

    /// Gets a reference to an [`ImmutableAccountAddressUnlockCondition`], if any.
    #[inline(always)]
    pub fn immutable_account_address(&self) -> Option<&ImmutableAccountAddressUnlockCondition> {
        self.get(ImmutableAccountAddressUnlockCondition::KIND)
            .map(UnlockCondition::as_immutable_account_address)
    }

    /// Returns the address to be unlocked.
    /// TODO take expiration deadzone into account
    #[inline(always)]
    pub fn locked_address<'a>(&'a self, address: &'a Address, slot_index: SlotIndex) -> &'a Address {
        self.expiration()
            .and_then(|e| e.return_address_expired(slot_index))
            .unwrap_or(address)
    }
}

impl StorageScore for UnlockConditions {
    fn storage_score(&self, params: StorageScoreParameters) -> u64 {
        self.iter().map(|uc| uc.storage_score(params)).sum()
    }
}

#[inline]
fn verify_unique_sorted<const VERIFY: bool>(unlock_conditions: &[UnlockCondition]) -> Result<(), Error> {
    if VERIFY && !is_unique_sorted(unlock_conditions.iter().map(UnlockCondition::kind)) {
        Err(Error::UnlockConditionsNotUniqueSorted)
    } else {
        Ok(())
    }
}

#[inline]
fn verify_unique_sorted_packable<const VERIFY: bool>(
    unlock_conditions: &[UnlockCondition],
    _: &ProtocolParameters,
) -> Result<(), Error> {
    verify_unique_sorted::<VERIFY>(unlock_conditions)
}

pub(crate) fn verify_allowed_unlock_conditions(
    unlock_conditions: &UnlockConditions,
    allowed_unlock_conditions: UnlockConditionFlags,
) -> Result<(), Error> {
    for (index, unlock_condition) in unlock_conditions.iter().enumerate() {
        if !allowed_unlock_conditions.contains(unlock_condition.flag()) {
            return Err(Error::UnallowedUnlockCondition {
                index,
                kind: unlock_condition.kind(),
            });
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn all_flags_present() {
        assert_eq!(
            UnlockConditionFlags::ALL_FLAGS,
            &[
                UnlockConditionFlags::ADDRESS,
                UnlockConditionFlags::STORAGE_DEPOSIT_RETURN,
                UnlockConditionFlags::TIMELOCK,
                UnlockConditionFlags::EXPIRATION,
                UnlockConditionFlags::STATE_CONTROLLER_ADDRESS,
                UnlockConditionFlags::GOVERNOR_ADDRESS,
                UnlockConditionFlags::IMMUTABLE_ACCOUNT_ADDRESS
            ]
        );
    }
}
