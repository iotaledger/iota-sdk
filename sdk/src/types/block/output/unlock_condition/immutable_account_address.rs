// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use derive_more::From;

use crate::types::block::address::AccountAddress;

/// Defines the permanent [`AccountAddress`] that owns this output.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, From, packable::Packable)]
pub struct ImmutableAccountAddressUnlockCondition(AccountAddress);

impl ImmutableAccountAddressUnlockCondition {
    /// The [`UnlockCondition`](crate::types::block::output::UnlockCondition) kind of an
    /// [`ImmutableAccountAddressUnlockCondition`].
    pub const KIND: u8 = 6;

    /// Creates a new [`ImmutableAccountAddressUnlockCondition`].
    #[inline(always)]
    pub fn new(address: impl Into<AccountAddress>) -> Self {
        Self(address.into())
    }

    /// Returns the account address of an [`ImmutableAccountAddressUnlockCondition`].
    pub fn address(&self) -> &AccountAddress {
        &self.0
    }
}

pub(super) mod dto {
    use serde::{Deserialize, Serialize};

    use super::*;

    #[derive(Serialize, Deserialize)]
    pub struct ImmutableAccountAddressUnlockConditionDto {
        #[serde(rename = "type")]
        pub kind: u8,
        pub address: AccountAddress,
    }

    impl From<&ImmutableAccountAddressUnlockCondition> for ImmutableAccountAddressUnlockConditionDto {
        fn from(value: &ImmutableAccountAddressUnlockCondition) -> Self {
            Self {
                kind: ImmutableAccountAddressUnlockCondition::KIND,
                address: value.0,
            }
        }
    }

    impl From<ImmutableAccountAddressUnlockConditionDto> for ImmutableAccountAddressUnlockCondition {
        fn from(value: ImmutableAccountAddressUnlockConditionDto) -> Self {
            Self(value.address)
        }
    }

    impl<'de> Deserialize<'de> for ImmutableAccountAddressUnlockCondition {
        fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            let dto = ImmutableAccountAddressUnlockConditionDto::deserialize(d)?;
            if dto.kind != Self::KIND {
                return Err(serde::de::Error::custom(format!(
                    "invalid immutable account adress unlock condition type: expected {}, found {}",
                    Self::KIND,
                    dto.kind
                )));
            }
            Ok(dto.into())
        }
    }

    impl Serialize for ImmutableAccountAddressUnlockCondition {
        fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            ImmutableAccountAddressUnlockConditionDto::from(self).serialize(s)
        }
    }
}
