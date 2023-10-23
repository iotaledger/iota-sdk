// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use derive_more::From;

use crate::types::block::{output::StorageScore, slot::SlotIndex, Error};

/// Defines a slot index until which the output can not be unlocked.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, From, packable::Packable)]
#[packable(unpack_error = Error)]
pub struct TimelockUnlockCondition(#[packable(verify_with = verify_slot_index)] SlotIndex);

impl TimelockUnlockCondition {
    /// The [`UnlockCondition`](crate::types::block::output::UnlockCondition) kind of a [`TimelockUnlockCondition`].
    pub const KIND: u8 = 2;

    /// Creates a new [`TimelockUnlockCondition`].
    #[inline(always)]
    pub fn new(slot_index: impl Into<SlotIndex>) -> Result<Self, Error> {
        let slot_index = slot_index.into();

        verify_slot_index::<true>(&slot_index, &())?;

        Ok(Self(slot_index))
    }

    /// Returns the slot index of a [`TimelockUnlockCondition`].
    #[inline(always)]
    pub fn slot_index(&self) -> SlotIndex {
        self.0
    }
}

impl StorageScore for TimelockUnlockCondition {}

#[inline]
fn verify_slot_index<const VERIFY: bool>(slot_index: &SlotIndex, _: &()) -> Result<(), Error> {
    if VERIFY && *slot_index == 0 {
        Err(Error::TimelockUnlockConditionZero)
    } else {
        Ok(())
    }
}

#[cfg(feature = "serde")]
pub(crate) mod dto {
    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::types::block::Error;

    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    struct TimelockUnlockConditionDto {
        #[serde(rename = "type")]
        kind: u8,
        slot_index: SlotIndex,
    }

    impl From<&TimelockUnlockCondition> for TimelockUnlockConditionDto {
        fn from(value: &TimelockUnlockCondition) -> Self {
            Self {
                kind: TimelockUnlockCondition::KIND,
                slot_index: value.slot_index(),
            }
        }
    }

    impl TryFrom<TimelockUnlockConditionDto> for TimelockUnlockCondition {
        type Error = Error;

        fn try_from(value: TimelockUnlockConditionDto) -> Result<Self, Error> {
            Self::new(value.slot_index).map_err(|_| Error::InvalidField("timelockUnlockCondition"))
        }
    }

    crate::impl_serde_typed_dto!(
        TimelockUnlockCondition,
        TimelockUnlockConditionDto,
        "timelock unlock condition"
    );
}
