// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::types::block::{unlock::UnlockIndex, Error};

/// An [`Unlock`](crate::types::block::unlock::Unlock) that refers to another unlock.
#[derive(Clone, Debug, Eq, PartialEq, Hash, packable::Packable)]
#[packable(unpack_error = Error, with = Error::InvalidReferenceIndex)]
pub struct ReferenceUnlock(UnlockIndex);

impl TryFrom<u16> for ReferenceUnlock {
    type Error = Error;

    fn try_from(index: u16) -> Result<Self, Self::Error> {
        Self::new(index)
    }
}

impl ReferenceUnlock {
    /// The [`Unlock`](crate::types::block::unlock::Unlock) kind of a [`ReferenceUnlock`].
    pub const KIND: u8 = 1;

    /// Creates a new [`ReferenceUnlock`].
    #[inline(always)]
    pub fn new(index: u16) -> Result<Self, Error> {
        index.try_into().map(Self).map_err(Error::InvalidReferenceIndex)
    }

    /// Return the index of a [`ReferenceUnlock`].
    #[inline(always)]
    pub fn index(&self) -> u16 {
        self.0.get()
    }
}

mod dto {
    use serde::{Deserialize, Serialize};

    use super::*;

    /// References a previous unlock in order to substitute the duplication of the same unlock data for inputs which
    /// unlock through the same data.
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
        type Error = Error;

        fn try_from(value: ReferenceUnlockDto) -> Result<Self, Self::Error> {
            Self::new(value.index)
        }
    }

    impl<'de> Deserialize<'de> for ReferenceUnlock {
        fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            let dto = ReferenceUnlockDto::deserialize(d)?;
            if dto.kind != Self::KIND {
                return Err(serde::de::Error::custom(format!(
                    "invalid reference unlock type: expected {}, found {}",
                    Self::KIND,
                    dto.kind
                )));
            }
            dto.try_into().map_err(serde::de::Error::custom)
        }
    }

    impl Serialize for ReferenceUnlock {
        fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            ReferenceUnlockDto::from(self).serialize(s)
        }
    }
}
