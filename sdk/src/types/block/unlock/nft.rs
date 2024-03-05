// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::block::{
    protocol::WorkScore,
    unlock::{UnlockError, UnlockIndex},
};

/// Points to the unlock of a consumed NFT output.
#[derive(Clone, Debug, Eq, PartialEq, Hash, packable::Packable)]
#[packable(unpack_error = UnlockError, with = UnlockError::NftIndex)]
pub struct NftUnlock(
    /// Index of input and unlock corresponding to an [`NftOutput`](crate::types::block::output::NftOutput).
    UnlockIndex,
);

impl NftUnlock {
    /// The [`Unlock`](crate::types::block::unlock::Unlock) kind of a [`NftUnlock`].
    pub const KIND: u8 = 4;

    /// Creates a new [`NftUnlock`].
    #[inline(always)]
    pub fn new(index: u16) -> Result<Self, UnlockError> {
        index.try_into().map(Self).map_err(UnlockError::NftIndex)
    }

    /// Return the index of a [`NftUnlock`].
    #[inline(always)]
    pub fn index(&self) -> u16 {
        self.0.get()
    }
}

impl WorkScore for NftUnlock {}

impl TryFrom<u16> for NftUnlock {
    type Error = UnlockError;

    fn try_from(index: u16) -> Result<Self, Self::Error> {
        Self::new(index)
    }
}

#[cfg(feature = "serde")]
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
        type Error = UnlockError;

        fn try_from(value: NftUnlockDto) -> Result<Self, Self::Error> {
            Self::new(value.index)
        }
    }

    crate::impl_serde_typed_dto!(NftUnlock, NftUnlockDto, "nft unlock");
}
