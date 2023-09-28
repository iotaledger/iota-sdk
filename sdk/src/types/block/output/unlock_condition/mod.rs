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
use packable::{
    bounded::BoundedU8,
    error::{UnpackError, UnpackErrorExt},
    packer::Packer,
    prefix::BoxedSlicePrefix,
    unpacker::Unpacker,
    Packable,
};

pub use self::{
    address::AddressUnlockCondition, expiration::ExpirationUnlockCondition,
    governor_address::GovernorAddressUnlockCondition,
    immutable_account_address::ImmutableAccountAddressUnlockCondition,
    state_controller_address::StateControllerAddressUnlockCondition,
    storage_deposit_return::StorageDepositReturnUnlockCondition, timelock::TimelockUnlockCondition,
};
use crate::types::block::{address::Address, create_bitflags, protocol::ProtocolParameters, slot::SlotIndex, Error};

///
#[derive(Clone, Eq, PartialEq, Hash, From)]
pub enum UnlockCondition {
    /// An address unlock condition.
    Address(AddressUnlockCondition),
    /// A storage deposit return unlock condition.
    StorageDepositReturn(StorageDepositReturnUnlockCondition),
    /// A timelock unlock condition.
    Timelock(TimelockUnlockCondition),
    /// An expiration unlock condition.
    Expiration(ExpirationUnlockCondition),
    /// A state controller address unlock condition.
    StateControllerAddress(StateControllerAddressUnlockCondition),
    /// A governor address unlock condition.
    GovernorAddress(GovernorAddressUnlockCondition),
    /// An immutable account address unlock condition.
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

impl core::fmt::Debug for UnlockCondition {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Address(unlock_condition) => unlock_condition.fmt(f),
            Self::StorageDepositReturn(unlock_condition) => unlock_condition.fmt(f),
            Self::Timelock(unlock_condition) => unlock_condition.fmt(f),
            Self::Expiration(unlock_condition) => unlock_condition.fmt(f),
            Self::StateControllerAddress(unlock_condition) => unlock_condition.fmt(f),
            Self::GovernorAddress(unlock_condition) => unlock_condition.fmt(f),
            Self::ImmutableAccountAddress(unlock_condition) => unlock_condition.fmt(f),
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

    /// Checks whether the unlock condition is an [`AddressUnlockCondition`].
    pub fn is_address(&self) -> bool {
        matches!(self, Self::Address(_))
    }

    /// Gets the unlock condition as an actual [`AddressUnlockCondition`].
    /// NOTE: Will panic if the unlock condition is not an [`AddressUnlockCondition`].
    pub fn as_address(&self) -> &AddressUnlockCondition {
        if let Self::Address(unlock_condition) = self {
            unlock_condition
        } else {
            panic!("invalid downcast of non-AddressUnlockCondition");
        }
    }

    /// Checks whether the unlock condition is a [`StorageDepositReturnUnlockCondition`].
    pub fn is_storage_deposit_return(&self) -> bool {
        matches!(self, Self::StorageDepositReturn(_))
    }

    /// Gets the unlock condition as an actual [`StorageDepositReturnUnlockCondition`].
    /// NOTE: Will panic if the unlock condition is not a [`StorageDepositReturnUnlockCondition`].
    pub fn as_storage_deposit_return(&self) -> &StorageDepositReturnUnlockCondition {
        if let Self::StorageDepositReturn(unlock_condition) = self {
            unlock_condition
        } else {
            panic!("invalid downcast of non-StorageDepositReturnUnlockCondition");
        }
    }

    /// Checks whether the unlock condition is a [`TimelockUnlockCondition`].
    pub fn is_timelock(&self) -> bool {
        matches!(self, Self::Timelock(_))
    }

    /// Gets the unlock condition as an actual [`TimelockUnlockCondition`].
    /// NOTE: Will panic if the unlock condition is not a [`TimelockUnlockCondition`].
    pub fn as_timelock(&self) -> &TimelockUnlockCondition {
        if let Self::Timelock(unlock_condition) = self {
            unlock_condition
        } else {
            panic!("invalid downcast of non-TimelockUnlockCondition");
        }
    }

    /// Checks whether the unlock condition is an [`ExpirationUnlockCondition`].
    pub fn is_expiration(&self) -> bool {
        matches!(self, Self::Expiration(_))
    }

    /// Gets the unlock condition as an actual [`ExpirationUnlockCondition`].
    /// NOTE: Will panic if the unlock condition is not an [`ExpirationUnlockCondition`].
    pub fn as_expiration(&self) -> &ExpirationUnlockCondition {
        if let Self::Expiration(unlock_condition) = self {
            unlock_condition
        } else {
            panic!("invalid downcast of non-ExpirationUnlockCondition");
        }
    }

    /// Checks whether the unlock condition is a [`StateControllerAddressUnlockCondition`].
    pub fn is_state_controller_address(&self) -> bool {
        matches!(self, Self::StateControllerAddress(_))
    }

    /// Gets the unlock condition as an actual [`StateControllerAddressUnlockCondition`].
    /// NOTE: Will panic if the unlock condition is not a [`StateControllerAddressUnlockCondition`].
    pub fn as_state_controller_address(&self) -> &StateControllerAddressUnlockCondition {
        if let Self::StateControllerAddress(unlock_condition) = self {
            unlock_condition
        } else {
            panic!("invalid downcast of non-StateControllerAddressUnlockCondition");
        }
    }

    /// Checks whether the unlock condition is a [`GovernorAddressUnlockCondition`].
    pub fn is_governor_address(&self) -> bool {
        matches!(self, Self::GovernorAddress(_))
    }

    /// Gets the unlock condition as an actual [`GovernorAddressUnlockCondition`].
    /// NOTE: Will panic if the unlock condition is not a [`GovernorAddressUnlockCondition`].
    pub fn as_governor_address(&self) -> &GovernorAddressUnlockCondition {
        if let Self::GovernorAddress(unlock_condition) = self {
            unlock_condition
        } else {
            panic!("invalid downcast of non-GovernorAddressUnlockCondition");
        }
    }

    /// Checks whether the unlock condition is an [`ImmutableAccountAddressUnlockCondition`].
    pub fn is_immutable_account_address(&self) -> bool {
        matches!(self, Self::ImmutableAccountAddress(_))
    }

    /// Gets the unlock condition as an actual [`ImmutableAccountAddressUnlockCondition`].
    /// NOTE: Will panic if the unlock condition is not an [`ImmutableAccountAddressUnlockCondition`].
    pub fn as_immutable_account_address(&self) -> &ImmutableAccountAddressUnlockCondition {
        if let Self::ImmutableAccountAddress(unlock_condition) = self {
            unlock_condition
        } else {
            panic!("invalid downcast of non-ImmutableAccountAddressUnlockCondition");
        }
    }
}

create_bitflags!(
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

impl Packable for UnlockCondition {
    type UnpackError = Error;
    type UnpackVisitor = ProtocolParameters;

    fn pack<P: Packer>(&self, packer: &mut P) -> Result<(), P::Error> {
        match self {
            Self::Address(unlock_condition) => {
                AddressUnlockCondition::KIND.pack(packer)?;
                unlock_condition.pack(packer)
            }
            Self::StorageDepositReturn(unlock_condition) => {
                StorageDepositReturnUnlockCondition::KIND.pack(packer)?;
                unlock_condition.pack(packer)
            }
            Self::Timelock(unlock_condition) => {
                TimelockUnlockCondition::KIND.pack(packer)?;
                unlock_condition.pack(packer)
            }
            Self::Expiration(unlock_condition) => {
                ExpirationUnlockCondition::KIND.pack(packer)?;
                unlock_condition.pack(packer)
            }
            Self::StateControllerAddress(unlock_condition) => {
                StateControllerAddressUnlockCondition::KIND.pack(packer)?;
                unlock_condition.pack(packer)
            }
            Self::GovernorAddress(unlock_condition) => {
                GovernorAddressUnlockCondition::KIND.pack(packer)?;
                unlock_condition.pack(packer)
            }
            Self::ImmutableAccountAddress(unlock_condition) => {
                ImmutableAccountAddressUnlockCondition::KIND.pack(packer)?;
                unlock_condition.pack(packer)
            }
        }?;

        Ok(())
    }

    fn unpack<U: Unpacker, const VERIFY: bool>(
        unpacker: &mut U,
        visitor: &Self::UnpackVisitor,
    ) -> Result<Self, UnpackError<Self::UnpackError, U::Error>> {
        Ok(match u8::unpack::<_, VERIFY>(unpacker, &()).coerce()? {
            AddressUnlockCondition::KIND => {
                Self::from(AddressUnlockCondition::unpack::<_, VERIFY>(unpacker, &()).coerce()?)
            }
            StorageDepositReturnUnlockCondition::KIND => {
                Self::from(StorageDepositReturnUnlockCondition::unpack::<_, VERIFY>(unpacker, visitor).coerce()?)
            }
            TimelockUnlockCondition::KIND => {
                Self::from(TimelockUnlockCondition::unpack::<_, VERIFY>(unpacker, &()).coerce()?)
            }
            ExpirationUnlockCondition::KIND => {
                Self::from(ExpirationUnlockCondition::unpack::<_, VERIFY>(unpacker, &()).coerce()?)
            }
            StateControllerAddressUnlockCondition::KIND => {
                Self::from(StateControllerAddressUnlockCondition::unpack::<_, VERIFY>(unpacker, &()).coerce()?)
            }
            GovernorAddressUnlockCondition::KIND => {
                Self::from(GovernorAddressUnlockCondition::unpack::<_, VERIFY>(unpacker, &()).coerce()?)
            }
            ImmutableAccountAddressUnlockCondition::KIND => {
                Self::from(ImmutableAccountAddressUnlockCondition::unpack::<_, VERIFY>(unpacker, &()).coerce()?)
            }
            k => return Err(Error::InvalidOutputKind(k)).map_err(UnpackError::Packable),
        })
    }
}

pub(crate) type UnlockConditionCount = BoundedU8<0, { UnlockConditions::COUNT_MAX }>;

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
    ///
    pub const COUNT_MAX: u8 = 7;

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

    /// Gets a reference to an [`ExpirationUnlockCondition`], if any.
    #[inline(always)]
    pub fn expiration(&self) -> Option<&ExpirationUnlockCondition> {
        self.get(ExpirationUnlockCondition::KIND)
            .map(UnlockCondition::as_expiration)
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
    #[inline(always)]
    pub fn locked_address<'a>(&'a self, address: &'a Address, slot_index: SlotIndex) -> &'a Address {
        self.expiration()
            .and_then(|e| e.return_address_expired(slot_index))
            .unwrap_or(address)
    }

    /// Returns whether a time lock exists and is still relevant.
    #[inline(always)]
    pub fn is_time_locked(&self, slot_index: impl Into<SlotIndex>) -> bool {
        let slot_index = slot_index.into();

        self.timelock()
            .map_or(false, |timelock| slot_index < timelock.slot_index())
    }

    /// Returns whether an expiration exists and is expired.
    #[inline(always)]
    pub fn is_expired(&self, slot_index: impl Into<SlotIndex>) -> bool {
        let slot_index = slot_index.into();

        self.expiration()
            .map_or(false, |expiration| slot_index >= expiration.slot_index())
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

#[cfg(feature = "serde")]
pub mod dto {
    use serde::{Deserialize, Serialize};

    pub use self::storage_deposit_return::dto::StorageDepositReturnUnlockConditionDto;
    use super::*;
    use crate::types::{block::Error, TryFromDto, ValidationParams};

    #[derive(Clone, Debug, Eq, PartialEq, From, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum UnlockConditionDto {
        /// An address unlock condition.
        Address(AddressUnlockCondition),
        /// A storage deposit return unlock condition.
        StorageDepositReturn(StorageDepositReturnUnlockConditionDto),
        /// A timelock unlock condition.
        Timelock(TimelockUnlockCondition),
        /// An expiration unlock condition.
        Expiration(ExpirationUnlockCondition),
        /// A state controller address unlock condition.
        StateControllerAddress(StateControllerAddressUnlockCondition),
        /// A governor address unlock condition.
        GovernorAddress(GovernorAddressUnlockCondition),
        /// An immutable account address unlock condition.
        ImmutableAccountAddress(ImmutableAccountAddressUnlockCondition),
    }

    impl From<&UnlockCondition> for UnlockConditionDto {
        fn from(value: &UnlockCondition) -> Self {
            match value {
                UnlockCondition::Address(v) => Self::Address(v.clone()),
                UnlockCondition::StorageDepositReturn(v) => Self::StorageDepositReturn(v.into()),
                UnlockCondition::Timelock(v) => Self::Timelock(*v),
                UnlockCondition::Expiration(v) => Self::Expiration(v.clone()),
                UnlockCondition::StateControllerAddress(v) => Self::StateControllerAddress(v.clone()),
                UnlockCondition::GovernorAddress(v) => Self::GovernorAddress(v.clone()),
                UnlockCondition::ImmutableAccountAddress(v) => Self::ImmutableAccountAddress(*v),
            }
        }
    }

    impl TryFromDto for UnlockCondition {
        type Dto = UnlockConditionDto;
        type Error = Error;

        fn try_from_dto_with_params_inner(dto: Self::Dto, params: ValidationParams<'_>) -> Result<Self, Self::Error> {
            Ok(match dto {
                UnlockConditionDto::Address(v) => Self::Address(v),
                UnlockConditionDto::StorageDepositReturn(v) => Self::StorageDepositReturn(
                    StorageDepositReturnUnlockCondition::try_from_dto_with_params_inner(v, params)?,
                ),
                UnlockConditionDto::Timelock(v) => Self::Timelock(v),
                UnlockConditionDto::Expiration(v) => Self::Expiration(v),
                UnlockConditionDto::StateControllerAddress(v) => Self::StateControllerAddress(v),
                UnlockConditionDto::GovernorAddress(v) => Self::GovernorAddress(v),
                UnlockConditionDto::ImmutableAccountAddress(v) => Self::ImmutableAccountAddress(v),
            })
        }
    }

    impl UnlockConditionDto {
        /// Return the unlock condition kind of a `UnlockConditionDto`.
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
    }
}
