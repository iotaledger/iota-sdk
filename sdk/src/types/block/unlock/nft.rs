// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::block::{unlock::UnlockIndex, Error};

/// Points to the unlock of a consumed NFT output.
#[derive(Clone, Debug, Eq, PartialEq, Hash, packable::Packable)]
#[packable(unpack_error = Error, with = Error::InvalidNftIndex)]
pub struct NftUnlock(
    /// Index of input and unlock corresponding to an [`NftOutput`](crate::types::block::output::NftOutput).
    UnlockIndex,
);

impl TryFrom<u16> for NftUnlock {
    type Error = Error;

    fn try_from(index: u16) -> Result<Self, Self::Error> {
        Self::new(index)
    }
}

impl NftUnlock {
    /// The [`Unlock`](crate::types::block::unlock::Unlock) kind of a [`NftUnlock`].
    pub const KIND: u8 = 3;

    /// Creates a new [`NftUnlock`].
    #[inline(always)]
    pub fn new(index: u16) -> Result<Self, Error> {
        index.try_into().map(Self).map_err(Error::InvalidNftIndex)
    }

    /// Return the index of a [`NftUnlock`].
    #[inline(always)]
    pub fn index(&self) -> u16 {
        self.0.get()
    }
}

#[cfg(feature = "serde_types")]
pub(crate) mod dto {
    use serde::{Deserialize, Serialize};

    use super::*;

    #[derive(Serialize, Deserialize)]
    struct NftUnlockDto {
        #[serde(rename = "type")]
        kind: u8,
        #[serde(rename = "reference")]
        index: u16,
    }

    impl From<&NftUnlock> for NftUnlockDto {
        fn from(value: &NftUnlock) -> Self {
            Self {
                kind: NftUnlock::KIND,
                index: value.0.get(),
            }
        }
    }

    impl TryFrom<NftUnlockDto> for NftUnlock {
        type Error = Error;

        fn try_from(value: NftUnlockDto) -> Result<Self, Self::Error> {
            Self::new(value.index)
        }
    }

    impl_serde_typed_dto!(NftUnlock, NftUnlockDto, "nft unlock");
}

#[cfg(feature = "json")]
mod json {
    use super::*;
    use crate::utils::json::{FromJson, JsonExt, ToJson, Value};

    impl ToJson for NftUnlock {
        fn to_json(&self) -> Value {
            crate::json! ({
                "type": Self::KIND,
                "reference": self.index()
            })
        }
    }

    impl FromJson for NftUnlock {
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
