// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::block::{
    protocol::WorkScore,
    unlock::{UnlockError, UnlockIndex},
};

/// An [`Unlock`](crate::types::block::unlock::Unlock) that refers to another unlock.
#[derive(Clone, Debug, Eq, PartialEq, Hash, packable::Packable)]
#[packable(unpack_error = UnlockError, with = UnlockError::ReferenceIndex)]
pub struct ReferenceUnlock(UnlockIndex);

impl TryFrom<u16> for ReferenceUnlock {
    type Error = UnlockError;

    fn try_from(index: u16) -> Result<Self, Self::Error> {
        Self::new(index)
    }
}

impl ReferenceUnlock {
    /// The [`Unlock`](crate::types::block::unlock::Unlock) kind of a [`ReferenceUnlock`].
    pub const KIND: u8 = 1;

    /// Creates a new [`ReferenceUnlock`].
    #[inline(always)]
    pub fn new(index: u16) -> Result<Self, UnlockError> {
        index.try_into().map(Self).map_err(UnlockError::ReferenceIndex)
    }

    /// Return the index of a [`ReferenceUnlock`].
    #[inline(always)]
    pub fn index(&self) -> u16 {
        self.0.get()
    }
}

impl WorkScore for ReferenceUnlock {}

#[cfg(feature = "serde")]
pub(crate) mod dto {
    use serde::{Deserialize, Serialize};

    use super::*;

    #[derive(Serialize, Deserialize)]
    struct ReferenceUnlockDto {
        #[serde(rename = "type")]
        kind: u8,
        #[serde(rename = "reference")]
        index: u16,
    }

    impl From<&ReferenceUnlock> for ReferenceUnlockDto {
        fn from(value: &ReferenceUnlock) -> Self {
            Self {
                kind: ReferenceUnlock::KIND,
                index: value.0.get(),
            }
        }
    }

    impl TryFrom<ReferenceUnlockDto> for ReferenceUnlock {
        type Error = UnlockError;

        fn try_from(value: ReferenceUnlockDto) -> Result<Self, Self::Error> {
            Self::new(value.index)
        }
    }

    crate::impl_serde_typed_dto!(ReferenceUnlock, ReferenceUnlockDto, "reference unlock");
}
