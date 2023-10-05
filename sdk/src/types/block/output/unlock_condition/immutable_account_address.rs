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

#[cfg(feature = "serde_types")]
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

#[cfg(feature = "json")]
mod json {
    use super::*;
    use crate::{
        types::block::Error,
        utils::json::{FromJson, JsonExt, ToJson, Value},
    };

    impl ToJson for ImmutableAccountAddressUnlockCondition {
        fn to_json(&self) -> Value {
            crate::json! ({
                "type": Self::KIND,
                "address": self.0,
            })
        }
    }

    impl FromJson for ImmutableAccountAddressUnlockCondition {
        type Error = Error;

        fn from_non_null_json(mut value: Value) -> Result<Self, Self::Error>
        where
            Self: Sized,
        {
            if value["type"] != Self::KIND {
                return Err(Error::invalid_type::<Self>(Self::KIND, &value["type"]));
            }
            Ok(Self::new(value["address"].take_value::<AccountAddress>()?))
        }
    }
}
