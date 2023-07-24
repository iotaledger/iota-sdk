// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use derive_more::From;

use crate::types::block::address::Address;

/// Defines the State Controller Address that owns this output, that is, it can unlock it with the proper Unlock in a
/// transaction that state transitions the account output.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, From, packable::Packable)]
pub struct StateControllerAddressUnlockCondition(Address);

impl StateControllerAddressUnlockCondition {
    /// The [`UnlockCondition`](crate::types::block::output::UnlockCondition) kind of an
    /// [`StateControllerAddressUnlockCondition`].
    pub const KIND: u8 = 4;

    /// Creates a new [`StateControllerAddressUnlockCondition`].
    #[inline(always)]
    pub fn new(address: impl Into<Address>) -> Self {
        Self(address.into())
    }

    /// Returns the address of a [`StateControllerAddressUnlockCondition`].
    #[inline(always)]
    pub fn address(&self) -> &Address {
        &self.0
    }
}

mod dto {
    use serde::{Deserialize, Serialize};

    use super::*;

    #[derive(Serialize, Deserialize)]
    struct StateControllerAddressUnlockConditionDto {
        #[serde(rename = "type")]
        kind: u8,
        address: Address,
    }

    impl From<&StateControllerAddressUnlockCondition> for StateControllerAddressUnlockConditionDto {
        fn from(value: &StateControllerAddressUnlockCondition) -> Self {
            Self {
                kind: StateControllerAddressUnlockCondition::KIND,
                address: value.address().into(),
            }
        }
    }

    impl From<StateControllerAddressUnlockConditionDto> for StateControllerAddressUnlockCondition {
        fn from(value: StateControllerAddressUnlockConditionDto) -> Self {
            Self(value.address)
        }
    }

    impl<'de> Deserialize<'de> for StateControllerAddressUnlockCondition {
        fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            let dto = StateControllerAddressUnlockConditionDto::deserialize(d)?;
            if dto.kind != Self::KIND {
                return Err(serde::de::Error::custom(format!(
                    "invalid state controller address unlock condition type: expected {}, found {}",
                    Self::KIND,
                    dto.kind
                )));
            }
            Ok(dto.into())
        }
    }

    impl Serialize for StateControllerAddressUnlockCondition {
        fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            StateControllerAddressUnlockConditionDto::from(self).serialize(s)
        }
    }
}
