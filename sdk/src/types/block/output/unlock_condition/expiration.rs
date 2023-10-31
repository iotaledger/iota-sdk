// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use derive_more::From;

use crate::types::block::{address::Address, slot::SlotIndex, Error};

/// Defines an expiration slot index. Before the slot index is reached, only the Address defined in the Address
/// Unlock Condition is allowed to unlock the output. Afterward, only the Return Address can unlock it.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, From, packable::Packable)]
pub struct ExpirationUnlockCondition {
    // The address that can unlock the expired output.
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
    pub fn new(return_address: impl Into<Address>, slot_index: impl Into<SlotIndex>) -> Result<Self, Error> {
        let slot_index = slot_index.into();

        verify_slot_index::<true>(&slot_index, &())?;

        Ok(Self {
            return_address: return_address.into(),
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

    /// Checks whether the expiration is expired.
    pub fn is_expired(&self, slot_index: impl Into<SlotIndex>, min_committable_age: impl Into<SlotIndex>) -> bool {
        (slot_index.into() + min_committable_age.into()) >= self.slot_index
    }

    /// Returns the return address if the condition has expired.
    pub fn return_address_expired<'a>(
        &'a self,
        address: &'a Address,
        slot_index: impl Into<SlotIndex>,
        min_committable_age: impl Into<SlotIndex>,
        max_committable_age: impl Into<SlotIndex>,
    ) -> Option<&'a Address> {
        let slot_index = slot_index.into();

        if self.slot_index() > (slot_index + max_committable_age.into()) {
            Some(address)
        } else if self.slot_index() <= (slot_index + min_committable_age.into()) {
            Some(&self.return_address)
        } else {
            None
        }
    }
}

#[inline]
fn verify_slot_index<const VERIFY: bool>(slot_index: &SlotIndex, _: &()) -> Result<(), Error> {
    if VERIFY && *slot_index == 0 {
        Err(Error::ExpirationUnlockConditionZero)
    } else {
        Ok(())
    }
}

#[cfg(feature = "serde")]
pub(crate) mod dto {
    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::types::block::Error;

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct ExpirationUnlockConditionDto {
        #[serde(rename = "type")]
        kind: u8,
        return_address: Address,
        slot_index: SlotIndex,
    }

    impl From<&ExpirationUnlockCondition> for ExpirationUnlockConditionDto {
        fn from(value: &ExpirationUnlockCondition) -> Self {
            Self {
                kind: ExpirationUnlockCondition::KIND,
                return_address: value.return_address().clone(),
                slot_index: value.slot_index(),
            }
        }
    }

    impl TryFrom<ExpirationUnlockConditionDto> for ExpirationUnlockCondition {
        type Error = Error;

        fn try_from(value: ExpirationUnlockConditionDto) -> Result<Self, Error> {
            Self::new(value.return_address, value.slot_index)
                .map_err(|_| Error::InvalidField("expirationUnlockCondition"))
        }
    }

    crate::impl_serde_typed_dto!(
        ExpirationUnlockCondition,
        ExpirationUnlockConditionDto,
        "expiration unlock condition"
    );
}
