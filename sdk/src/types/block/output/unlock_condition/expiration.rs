// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use derive_more::From;

use crate::types::block::{
    address::{Address, AddressError},
    output::{unlock_condition::UnlockConditionError, StorageScore, StorageScoreParameters},
    protocol::CommittableAgeRange,
    slot::SlotIndex,
};

/// Defines an expiration slot index. Before the slot index is reached, only the Address defined in the Address
/// Unlock Condition is allowed to unlock the output. Afterward, only the Return Address can unlock it.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, From, packable::Packable)]
#[packable(unpack_error = UnlockConditionError)]
pub struct ExpirationUnlockCondition {
    // The address that can unlock the expired output.
    #[packable(verify_with = verify_return_address)]
    return_address: Address,
    /// The slot index that determines when the associated output expires.
    #[packable(verify_with = verify_slot_index)]
    slot_index: SlotIndex,
}

impl ExpirationUnlockCondition {
    /// The [`UnlockCondition`](crate::types::block::output::UnlockCondition) kind of an [`ExpirationUnlockCondition`].
    pub const KIND: u8 = 3;

    /// Creates a new [`ExpirationUnlockCondition`].
    #[inline(always)]
    pub fn new(
        return_address: impl Into<Address>,
        slot_index: impl Into<SlotIndex>,
    ) -> Result<Self, UnlockConditionError> {
        let slot_index = slot_index.into();
        let return_address = return_address.into();

        verify_slot_index(&slot_index)?;
        verify_return_address(&return_address)?;

        Ok(Self {
            return_address,
            slot_index,
        })
    }

    /// Returns the return address of a [`ExpirationUnlockCondition`].
    #[inline(always)]
    pub fn return_address(&self) -> &Address {
        &self.return_address
    }

    /// Returns the slot index of a [`ExpirationUnlockCondition`].
    #[inline(always)]
    pub fn slot_index(&self) -> SlotIndex {
        self.slot_index
    }

    /// Checks whether the expiration is expired. If None is returned, then expiration is in the deadzone where it can't
    /// be unlocked.
    pub fn is_expired(
        &self,
        slot_index: impl Into<SlotIndex>,
        committable_age_range: CommittableAgeRange,
    ) -> Option<bool> {
        let slot_index = slot_index.into();

        if self.slot_index() > (slot_index + committable_age_range.max) {
            Some(false)
        } else if self.slot_index() <= (slot_index + committable_age_range.min) {
            Some(true)
        } else {
            None
        }
    }

    /// Returns the address that can unlock the output. If there is an expiration unlock condition, then there is a
    /// small interval around the expiration slot index in which no address can unlock the output.
    pub fn return_address_expired<'a>(
        &'a self,
        address: &'a Address,
        slot_index: impl Into<SlotIndex>,
        committable_age_range: CommittableAgeRange,
    ) -> Option<&'a Address> {
        self.is_expired(slot_index, committable_age_range).map(
            |expired| {
                if expired { &self.return_address } else { address }
            },
        )
    }
}

impl StorageScore for ExpirationUnlockCondition {
    fn storage_score(&self, params: StorageScoreParameters) -> u64 {
        self.return_address().storage_score(params)
    }
}

#[inline]
fn verify_return_address(return_address: &Address) -> Result<(), UnlockConditionError> {
    if return_address.is_implicit_account_creation() {
        Err(AddressError::Kind(return_address.kind()).into())
    } else {
        Ok(())
    }
}

#[inline]
fn verify_slot_index(slot_index: &SlotIndex) -> Result<(), UnlockConditionError> {
    if *slot_index == 0 {
        Err(UnlockConditionError::ExpirationZero)
    } else {
        Ok(())
    }
}

#[cfg(feature = "serde")]
pub(crate) mod dto {
    use serde::{Deserialize, Serialize};

    use super::*;

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct ExpirationUnlockConditionDto {
        #[serde(rename = "type")]
        kind: u8,
        return_address: Address,
        slot: SlotIndex,
    }

    impl From<&ExpirationUnlockCondition> for ExpirationUnlockConditionDto {
        fn from(value: &ExpirationUnlockCondition) -> Self {
            Self {
                kind: ExpirationUnlockCondition::KIND,
                return_address: value.return_address().clone(),
                slot: value.slot_index(),
            }
        }
    }

    impl TryFrom<ExpirationUnlockConditionDto> for ExpirationUnlockCondition {
        type Error = UnlockConditionError;

        fn try_from(value: ExpirationUnlockConditionDto) -> Result<Self, UnlockConditionError> {
            Self::new(value.return_address, value.slot)
        }
    }

    crate::impl_serde_typed_dto!(
        ExpirationUnlockCondition,
        ExpirationUnlockConditionDto,
        "expiration unlock condition"
    );
}
