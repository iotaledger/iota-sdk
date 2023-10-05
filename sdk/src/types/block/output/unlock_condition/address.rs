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

#[cfg(feature = "serde_types")]
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
                address: value.0,
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

#[cfg(feature = "json")]
mod json {
    use super::*;
    use crate::{
        types::block::Error,
        utils::json::{FromJson, JsonExt, ToJson, Value},
    };

    impl ToJson for AddressUnlockCondition {
        fn to_json(&self) -> Value {
            crate::json! ({
                "type": Self::KIND,
                "address": self.0,
            })
        }
    }

    impl FromJson for AddressUnlockCondition {
        type Error = Error;

        fn from_non_null_json(mut value: Value) -> Result<Self, Self::Error>
        where
            Self: Sized,
        {
            if value["type"] != Self::KIND {
                return Err(Error::invalid_type::<Self>(Self::KIND, &value["type"]));
            }
            Ok(Self::new(value["address"].take_value::<Address>()?))
        }
    }
}
