// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use derive_more::From;

use crate::types::block::{address::Address, slot::SlotIndex, Error};

/// Defines an expiration slot index. Before the slot index is reached, only the Address defined in the Address
/// Unlock Condition is allowed to unlock the output. Afterward, only the Return Address can unlock it.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, From, packable::Packable)]
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

    /// Returns the return address if the condition has expired.
    pub fn return_address_expired(&self, slot_index: SlotIndex) -> Option<&Address> {
        if slot_index >= self.slot_index() {
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

#[cfg(feature = "serde_types")]
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
                return_address: *value.return_address(),
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

    impl_serde_typed_dto!(
        ExpirationUnlockCondition,
        ExpirationUnlockConditionDto,
        "expiration unlock condition"
    );
}

#[cfg(feature = "json")]
mod json {
    use super::*;
    use crate::{
        types::block::Error,
        utils::json::{FromJson, JsonExt, ToJson, Value},
    };

    impl ToJson for ExpirationUnlockCondition {
        fn to_json(&self) -> Value {
            crate::json! ({
                "type": Self::KIND,
                "returnAddress": self.return_address(),
                "slotIndex": self.slot_index,
            })
        }
    }

    impl FromJson for ExpirationUnlockCondition {
        type Error = Error;

        fn from_non_null_json(mut value: Value) -> Result<Self, Self::Error>
        where
            Self: Sized,
        {
            if value["type"] != Self::KIND {
                return Err(Error::invalid_type::<Self>(Self::KIND, &value["type"]));
            }
            Self::new(
                value["returnAddress"].take_value::<Address>()?,
                value["slotIndex"].take_value::<SlotIndex>()?,
            )
        }
    }
}
