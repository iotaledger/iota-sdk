// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::block::{unlock::UnlockIndex, Error};

/// Points to the unlock of a consumed account output.
#[derive(Clone, Debug, Eq, PartialEq, Hash, packable::Packable)]
#[packable(unpack_error = Error, with = Error::InvalidAccountIndex)]
pub struct AccountUnlock(
    /// Index of input and unlock corresponding to an [`AccountOutput`](crate::types::block::output::AccountOutput).
    UnlockIndex,
);

impl TryFrom<u16> for AccountUnlock {
    type Error = Error;

    fn try_from(index: u16) -> Result<Self, Self::Error> {
        Self::new(index)
    }
}

impl AccountUnlock {
    /// The [`Unlock`](crate::types::block::unlock::Unlock) kind of an [`AccountUnlock`].
    pub const KIND: u8 = 2;

    /// Creates a new [`AccountUnlock`].
    #[inline(always)]
    pub fn new(index: u16) -> Result<Self, Error> {
        index.try_into().map(Self).map_err(Error::InvalidAccountIndex)
    }

    /// Return the index of an [`AccountUnlock`].
    #[inline(always)]
    pub fn index(&self) -> u16 {
        self.0.get()
    }
}

mod dto {
    use serde::{Deserialize, Serialize};

    use super::*;

    #[derive(Serialize, Deserialize)]
    struct AccountUnlockDto {
        #[serde(rename = "type")]
        kind: u8,
        #[serde(rename = "reference")]
        index: u16,
    }

    impl From<&AccountUnlock> for AccountUnlockDto {
        fn from(value: &AccountUnlock) -> Self {
            Self {
                kind: AccountUnlock::KIND,
                index: value.0.get(),
            }
        }
    }

    impl TryFrom<AccountUnlockDto> for AccountUnlock {
        type Error = Error;

        fn try_from(value: AccountUnlockDto) -> Result<Self, Self::Error> {
            Self::new(value.index)
        }
    }

    impl_serde_typed_dto!(AccountUnlock, AccountUnlockDto, "account unlock");
}

#[cfg(feature = "json")]
mod json {
    use super::*;
    use crate::utils::json::{FromJson, TakeValue, ToJson, Value};

    impl ToJson for AccountUnlock {
        fn to_json(&self) -> Value {
            crate::json! ({
                "type": Self::KIND,
                "reference": self.index()
            })
        }
    }

    impl FromJson for AccountUnlock {
        type Error = Error;

        fn from_non_null_json(mut value: Value) -> Result<Self, Self::Error>
        where
            Self: Sized,
        {
            if value["type"] != Self::KIND {
                return Err(Error::invalid_type::<Self>(Self::KIND, &value["type"]));
            }
            Self::new(value["reference"].take_value()?)
        }
    }
}
