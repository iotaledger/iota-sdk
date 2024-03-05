// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use derive_more::From;

use crate::types::block::{
    output::{unlock_condition::UnlockConditionError, StorageScore},
    slot::SlotIndex,
};

/// Defines a slot index until which the output can not be unlocked.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, From, packable::Packable)]
#[packable(unpack_error = UnlockConditionError)]
pub struct TimelockUnlockCondition(#[packable(verify_with = verify_slot_index)] SlotIndex);

impl TimelockUnlockCondition {
    /// The [`UnlockCondition`](crate::types::block::output::UnlockCondition) kind of a [`TimelockUnlockCondition`].
    pub const KIND: u8 = 2;

    /// Creates a new [`TimelockUnlockCondition`].
    #[inline(always)]
    pub fn new(slot_index: impl Into<SlotIndex>) -> Result<Self, UnlockConditionError> {
        let slot_index = slot_index.into();

        verify_slot_index(&slot_index)?;

        Ok(Self(slot_index))
    }

    /// Returns the slot index of a [`TimelockUnlockCondition`].
    #[inline(always)]
    pub fn slot_index(&self) -> SlotIndex {
        self.0
    }

    /// Checks whether the timelock is still relevant.
    pub fn is_timelocked(&self, slot_index: impl Into<SlotIndex>, min_committable_age: impl Into<SlotIndex>) -> bool {
        (slot_index.into() + min_committable_age.into()) < self.0
    }
}

impl StorageScore for TimelockUnlockCondition {}

#[inline]
fn verify_slot_index(slot_index: &SlotIndex) -> Result<(), UnlockConditionError> {
    if *slot_index == 0 {
        Err(UnlockConditionError::TimelockZero)
    } else {
        Ok(())
    }
}

#[cfg(feature = "serde")]
pub(crate) mod dto {
    use serde::{Deserialize, Serialize};

    use super::*;

    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct TimelockUnlockConditionDto {
        #[serde(rename = "type")]
        kind: u8,
        slot: SlotIndex,
    }

    impl From<&TimelockUnlockCondition> for TimelockUnlockConditionDto {
        fn from(value: &TimelockUnlockCondition) -> Self {
            Self {
                kind: TimelockUnlockCondition::KIND,
                slot: value.slot_index(),
            }
        }
    }

    impl TryFrom<TimelockUnlockConditionDto> for TimelockUnlockCondition {
        type Error = UnlockConditionError;

        fn try_from(value: TimelockUnlockConditionDto) -> Result<Self, UnlockConditionError> {
            Self::new(value.slot)
        }
    }

    crate::impl_serde_typed_dto!(
        TimelockUnlockCondition,
        TimelockUnlockConditionDto,
        "timelock unlock condition"
    );
}
