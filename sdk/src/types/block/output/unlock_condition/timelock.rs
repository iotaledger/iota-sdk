// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use derive_more::From;

use crate::types::block::Error;

/// Defines a unix timestamp until which the output can not be unlocked.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, From, packable::Packable)]
#[packable(unpack_error = Error)]
pub struct TimelockUnlockCondition(#[packable(verify_with = verify_timestamp)] u32);

impl TimelockUnlockCondition {
    /// The [`UnlockCondition`](crate::types::block::output::UnlockCondition) kind of a [`TimelockUnlockCondition`].
    pub const KIND: u8 = 2;

    /// Creates a new [`TimelockUnlockCondition`].
    #[inline(always)]
    pub fn new(timestamp: u32) -> Result<Self, Error> {
        verify_timestamp::<true>(&timestamp, &())?;

        Ok(Self(timestamp))
    }

    /// Returns the timestamp of a [`TimelockUnlockCondition`].
    #[inline(always)]
    pub fn timestamp(&self) -> u32 {
        self.0
    }
}

#[inline]
fn verify_timestamp<const VERIFY: bool>(timestamp: &u32, _: &()) -> Result<(), Error> {
    if VERIFY && *timestamp == 0 {
        Err(Error::TimelockUnlockConditionZero)
    } else {
        Ok(())
    }
}

mod dto {
    use alloc::format;

    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::types::block::Error;

    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    struct TimelockUnlockConditionDto {
        #[serde(rename = "type")]
        kind: u8,
        #[serde(rename = "unixTime")]
        timestamp: u32,
    }

    impl From<&TimelockUnlockCondition> for TimelockUnlockConditionDto {
        fn from(value: &TimelockUnlockCondition) -> Self {
            Self {
                kind: TimelockUnlockCondition::KIND,
                timestamp: value.timestamp(),
            }
        }
    }

    impl TryFrom<TimelockUnlockConditionDto> for TimelockUnlockCondition {
        type Error = Error;

        fn try_from(value: TimelockUnlockConditionDto) -> Result<Self, Error> {
            Self::new(value.timestamp).map_err(|_| Error::InvalidField("timelockUnlockCondition"))
        }
    }

    impl<'de> Deserialize<'de> for TimelockUnlockCondition {
        fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            let dto = TimelockUnlockConditionDto::deserialize(d)?;
            if dto.kind != Self::KIND {
                return Err(serde::de::Error::custom(format!(
                    "invalid timelock unlock condition type: expected {}, found {}",
                    Self::KIND,
                    dto.kind
                )));
            }
            dto.try_into().map_err(serde::de::Error::custom)
        }
    }

    impl Serialize for TimelockUnlockCondition {
        fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            TimelockUnlockConditionDto::from(self).serialize(s)
        }
    }
}
