// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use derive_more::From;

use crate::types::block::{
    address::Address,
    output::{storage_score::StorageScoreParameters, StorageScore},
};

/// Defines the Address that owns this output, that is, it can unlock it with the proper Unlock in a transaction.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, From, packable::Packable)]
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

impl StorageScore for AddressUnlockCondition {
    fn storage_score(&self, params: StorageScoreParameters) -> u64 {
        self.address().storage_score(params)
    }
}

#[cfg(feature = "serde")]
pub(crate) mod dto {
    use serde::{Deserialize, Serialize};

    use super::*;

    #[derive(Serialize, Deserialize)]
    struct AddressUnlockConditionDto {
        #[serde(rename = "type")]
        kind: u8,
        address: Address,
    }

    impl From<&AddressUnlockCondition> for AddressUnlockConditionDto {
        fn from(value: &AddressUnlockCondition) -> Self {
            Self {
                kind: AddressUnlockCondition::KIND,
                address: value.0.clone(),
            }
        }
    }

    impl From<AddressUnlockConditionDto> for AddressUnlockCondition {
        fn from(value: AddressUnlockConditionDto) -> Self {
            Self(value.address)
        }
    }

    impl_serde_typed_dto!(
        AddressUnlockCondition,
        AddressUnlockConditionDto,
        "address unlock condition"
    );
}
