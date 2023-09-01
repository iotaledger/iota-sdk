// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use derive_more::From;

use crate::types::block::address::Address;

/// Defines the Governor Address that owns this output, that is, it can unlock it with the proper Unlock in a
/// transaction that governance transitions the account output.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, From, packable::Packable)]
pub struct GovernorAddressUnlockCondition(Address);

impl GovernorAddressUnlockCondition {
    /// The [`UnlockCondition`](crate::types::block::output::UnlockCondition) kind of an
    /// [`GovernorAddressUnlockCondition`].
    pub const KIND: u8 = 5;

    /// Creates a new [`GovernorAddressUnlockCondition`].
    #[inline(always)]
    pub fn new(address: impl Into<Address>) -> Self {
        Self(address.into())
    }

    /// Returns the address of a [`GovernorAddressUnlockCondition`].
    #[inline(always)]
    pub fn address(&self) -> &Address {
        &self.0
    }
}

#[cfg(feature = "serde")]
pub(crate) mod dto {
    use serde::{Deserialize, Serialize};

    use super::*;

    #[derive(Serialize, Deserialize)]
    struct GovernorAddressUnlockConditionDto {
        #[serde(rename = "type")]
        kind: u8,
        address: Address,
    }

    impl From<&GovernorAddressUnlockCondition> for GovernorAddressUnlockConditionDto {
        fn from(value: &GovernorAddressUnlockCondition) -> Self {
            Self {
                kind: GovernorAddressUnlockCondition::KIND,
                address: value.0,
            }
        }
    }

    impl From<GovernorAddressUnlockConditionDto> for GovernorAddressUnlockCondition {
        fn from(value: GovernorAddressUnlockConditionDto) -> Self {
            Self(value.address)
        }
    }

    impl_serde_typed_dto!(
        GovernorAddressUnlockCondition,
        GovernorAddressUnlockConditionDto,
        "governor address unlock condition"
    );
}
