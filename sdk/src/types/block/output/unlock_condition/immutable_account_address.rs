// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use derive_more::From;

use crate::types::block::{
    address::AccountAddress,
    output::{RentParameters, StorageScore},
};

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

impl StorageScore for ImmutableAccountAddressUnlockCondition {
    fn storage_score(&self, params: RentParameters) -> u64 {
        self.address().storage_score(params)
    }
}

#[cfg(feature = "serde")]
mod dto {
    use serde::{Deserialize, Serialize};

    use super::*;

    #[derive(Serialize, Deserialize)]
    struct ImmutableAccountAddressUnlockConditionDto {
        #[serde(rename = "type")]
        kind: u8,
        address: AccountAddress,
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

    impl_serde_typed_dto!(
        ImmutableAccountAddressUnlockCondition,
        ImmutableAccountAddressUnlockConditionDto,
        "immutable account adress unlock condition"
    );
}
