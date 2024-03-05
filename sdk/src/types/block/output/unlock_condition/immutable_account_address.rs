// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use derive_more::From;

use crate::types::block::{
    address::{AccountAddress, Address, AddressError},
    output::{unlock_condition::UnlockConditionError, StorageScore, StorageScoreParameters},
};

/// Defines the permanent [`AccountAddress`] that owns this output.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, From, packable::Packable)]
#[packable(unpack_error = UnlockConditionError)]
pub struct ImmutableAccountAddressUnlockCondition(#[packable(verify_with = verify_address)] Address);

impl ImmutableAccountAddressUnlockCondition {
    /// The [`UnlockCondition`](crate::types::block::output::UnlockCondition) kind of an
    /// [`ImmutableAccountAddressUnlockCondition`].
    pub const KIND: u8 = 6;

    /// Creates a new [`ImmutableAccountAddressUnlockCondition`].
    #[inline(always)]
    pub fn new(address: impl Into<AccountAddress>) -> Self {
        Self(Address::from(address.into()))
    }

    /// Returns the account address of an [`ImmutableAccountAddressUnlockCondition`].
    pub fn address(&self) -> &AccountAddress {
        self.0.as_account()
    }
}

impl StorageScore for ImmutableAccountAddressUnlockCondition {
    fn storage_score(&self, params: StorageScoreParameters) -> u64 {
        self.address().storage_score(params)
    }
}

#[inline]
fn verify_address(address: &Address) -> Result<(), UnlockConditionError> {
    if !address.is_account() {
        Err(AddressError::Kind(address.kind()).into())
    } else {
        Ok(())
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
        address: Address,
    }

    impl From<&ImmutableAccountAddressUnlockCondition> for ImmutableAccountAddressUnlockConditionDto {
        fn from(value: &ImmutableAccountAddressUnlockCondition) -> Self {
            Self {
                kind: ImmutableAccountAddressUnlockCondition::KIND,
                address: value.0.clone(),
            }
        }
    }

    impl From<ImmutableAccountAddressUnlockConditionDto> for ImmutableAccountAddressUnlockCondition {
        fn from(value: ImmutableAccountAddressUnlockConditionDto) -> Self {
            Self(value.address)
        }
    }

    crate::impl_serde_typed_dto!(
        ImmutableAccountAddressUnlockCondition,
        ImmutableAccountAddressUnlockConditionDto,
        "immutable account adress unlock condition"
    );
}
