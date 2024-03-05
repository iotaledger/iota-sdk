// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::block::{
    address::{Address, AddressError},
    output::{
        storage_score::{StorageScore, StorageScoreParameters},
        unlock_condition::UnlockConditionError,
    },
};

/// Defines the amount of IOTAs used as storage deposit that have to be returned to the return [`Address`].
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, packable::Packable)]
#[packable(unpack_error = UnlockConditionError)]
pub struct StorageDepositReturnUnlockCondition {
    // The [`Address`] to return the amount to.
    #[packable(verify_with = verify_return_address)]
    return_address: Address,
    // Amount of IOTA coins the consuming transaction should deposit to `return_address`.
    amount: u64,
}

impl StorageDepositReturnUnlockCondition {
    /// The [`UnlockCondition`](crate::types::block::output::UnlockCondition) kind of a
    /// [`StorageDepositReturnUnlockCondition`].
    pub const KIND: u8 = 1;

    /// Creates a new [`StorageDepositReturnUnlockCondition`].
    #[inline(always)]
    pub fn new(return_address: impl Into<Address>, amount: u64) -> Result<Self, UnlockConditionError> {
        let return_address = return_address.into();

        verify_return_address(&return_address)?;

        Ok(Self { return_address, amount })
    }

    /// Returns the return address.
    #[inline(always)]
    pub fn return_address(&self) -> &Address {
        &self.return_address
    }

    /// Returns the amount.
    #[inline(always)]
    pub fn amount(&self) -> u64 {
        self.amount
    }
}

impl StorageScore for StorageDepositReturnUnlockCondition {
    fn storage_score(&self, params: StorageScoreParameters) -> u64 {
        self.return_address().storage_score(params)
    }
}

#[inline]
fn verify_return_address(return_address: &Address) -> Result<(), UnlockConditionError> {
    if return_address.is_implicit_account_creation() {
        Err(AddressError::Kind(return_address.kind()).into())
    } else {
        Ok(())
    }
}

#[cfg(feature = "serde")]
mod dto {
    use alloc::format;

    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::utils::serde::string;

    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct StorageDepositReturnUnlockConditionDto {
        #[serde(rename = "type", deserialize_with = "deserialize_kind")]
        pub kind: u8,
        pub return_address: Address,
        #[serde(with = "string")]
        pub amount: u64,
    }

    fn deserialize_kind<'de, D>(d: D) -> Result<u8, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let kind = u8::deserialize(d)?;
        if kind != StorageDepositReturnUnlockCondition::KIND {
            return Err(serde::de::Error::custom(format!(
                "invalid storage deposit return unlock condition type: expected {}, found {}",
                StorageDepositReturnUnlockCondition::KIND,
                kind
            )));
        }
        Ok(kind)
    }

    impl From<&StorageDepositReturnUnlockCondition> for StorageDepositReturnUnlockConditionDto {
        fn from(value: &StorageDepositReturnUnlockCondition) -> Self {
            Self {
                kind: StorageDepositReturnUnlockCondition::KIND,
                return_address: value.return_address.clone(),
                amount: value.amount,
            }
        }
    }

    impl From<StorageDepositReturnUnlockConditionDto> for StorageDepositReturnUnlockCondition {
        fn from(dto: StorageDepositReturnUnlockConditionDto) -> Self {
            Self {
                return_address: dto.return_address,
                amount: dto.amount,
            }
        }
    }

    crate::impl_serde_typed_dto!(
        StorageDepositReturnUnlockCondition,
        StorageDepositReturnUnlockConditionDto,
        "storage deposit return unlock condition"
    );
}
