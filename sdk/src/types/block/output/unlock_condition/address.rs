// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use derive_more::From;

use crate::types::block::address::Address;

/// Defines the Address that owns this output, that is, it can unlock it with the proper Unlock in a transaction.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, From, packable::Packable)]
pub struct AddressUnlockCondition(Address);

impl AddressUnlockCondition {
    /// The [`UnlockCondition`](crate::types::block::output::UnlockCondition) kind of an [`AddressUnlockCondition`].
    pub const KIND: u8 = 0;

    /// Creates a new [`AddressUnlockCondition`].
    #[inline(always)]
    pub fn new(address: impl Into<Address>) -> Self {
        Self(address.into())
    }

    /// Returns the address of a [`AddressUnlockCondition`].
    #[inline(always)]
    pub fn address(&self) -> &Address {
        &self.0
    }
}

pub(super) mod dto {
    use serde::{Deserialize, Serialize};

    use super::*;

    #[derive(Serialize, Deserialize)]
    pub struct AddressUnlockConditionDto {
        #[serde(rename = "type")]
        pub kind: u8,
        pub address: Address,
    }

    impl From<&AddressUnlockCondition> for AddressUnlockConditionDto {
        fn from(value: &AddressUnlockCondition) -> Self {
            Self {
                kind: AddressUnlockCondition::KIND,
                address: value.0,
            }
        }
    }

    impl From<AddressUnlockConditionDto> for AddressUnlockCondition {
        fn from(value: AddressUnlockConditionDto) -> Self {
            Self(value.address)
        }
    }

    impl<'de> Deserialize<'de> for AddressUnlockCondition {
        fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            let dto = AddressUnlockConditionDto::deserialize(d)?;
            if dto.kind != Self::KIND {
                return Err(serde::de::Error::custom(format!(
                    "invalid address unlock condition type: expected {}, found {}",
                    Self::KIND,
                    dto.kind
                )));
            }
            Ok(dto.into())
        }
    }

    impl Serialize for AddressUnlockCondition {
        fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            AddressUnlockConditionDto::from(self).serialize(s)
        }
    }
}
