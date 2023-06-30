// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::block::{unlock::UnlockIndex, Error};

/// Points to the unlock of a consumed alias output.
#[derive(Clone, Debug, Eq, PartialEq, Hash, packable::Packable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[packable(unpack_error = Error, with = Error::InvalidAliasIndex)]
pub struct AccountUnlock(
    /// Index of input and unlock corresponding to an [`AliasOutput`](crate::types::block::output::AliasOutput).
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
        index.try_into().map(Self).map_err(Error::InvalidAliasIndex)
    }

    /// Return the index of an [`AccountUnlock`].
    #[inline(always)]
    pub fn index(&self) -> u16 {
        self.0.get()
    }
}

#[allow(missing_docs)]
pub mod dto {
    use serde::{Deserialize, Serialize};

    /// Points to the unlock of a consumed alias output.
    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    pub struct AccountUnlockDto {
        #[serde(rename = "type")]
        pub kind: u8,
        #[serde(rename = "reference")]
        pub index: u16,
    }
}
