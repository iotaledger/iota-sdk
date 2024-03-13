// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::block::{
    protocol::WorkScore,
    unlock::{UnlockError, UnlockIndex},
};

/// Points to the unlock of a consumed anchor output.
#[derive(Clone, Debug, Eq, PartialEq, Hash, packable::Packable)]
#[packable(unpack_error = UnlockError, with = UnlockError::AnchorIndex)]
pub struct AnchorUnlock(
    /// Index of input and unlock corresponding to an [`AnchorOutput`](crate::types::block::output::AnchorOutput).
    UnlockIndex,
);

impl AnchorUnlock {
    /// The [`Unlock`](crate::types::block::unlock::Unlock) kind of an [`AnchorUnlock`].
    pub const KIND: u8 = 3;

    /// Creates a new [`AnchorUnlock`].
    #[inline(always)]
    pub fn new(index: u16) -> Result<Self, UnlockError> {
        index.try_into().map(Self).map_err(UnlockError::AnchorIndex)
    }

    /// Return the index of an [`AnchorUnlock`].
    #[inline(always)]
    pub fn index(&self) -> u16 {
        self.0.get()
    }
}

impl WorkScore for AnchorUnlock {}

impl TryFrom<u16> for AnchorUnlock {
    type Error = UnlockError;

    fn try_from(index: u16) -> Result<Self, Self::Error> {
        Self::new(index)
    }
}

#[cfg(feature = "serde")]
mod dto {
    use serde::{Deserialize, Serialize};

    use super::*;

    #[derive(Serialize, Deserialize)]
    struct AnchorUnlockDto {
        #[serde(rename = "type")]
        kind: u8,
        #[serde(rename = "reference")]
        index: u16,
    }

    impl From<&AnchorUnlock> for AnchorUnlockDto {
        fn from(value: &AnchorUnlock) -> Self {
            Self {
                kind: AnchorUnlock::KIND,
                index: value.0.get(),
            }
        }
    }

    impl TryFrom<AnchorUnlockDto> for AnchorUnlock {
        type Error = UnlockError;

        fn try_from(value: AnchorUnlockDto) -> Result<Self, Self::Error> {
            Self::new(value.index)
        }
    }

    crate::impl_serde_typed_dto!(AnchorUnlock, AnchorUnlockDto, "anchor unlock");
}
