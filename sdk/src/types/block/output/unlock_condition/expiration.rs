// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use derive_more::From;

use crate::types::block::{address::Address, Error};

/// Defines a unix time until which only Address, defined in Address Unlock Condition, is allowed to unlock the output.
/// After or at the unix time, only Return Address can unlock it.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, From, packable::Packable)]
pub struct ExpirationUnlockCondition {
    // The address that can unlock the expired output.
    return_address: Address,
    // Before this unix time, seconds since unix epoch,
    // [`AddressUnlockCondition`](crate::types::unlock_condition::AddressUnlockCondition) is allowed to unlock the
    // output. After that, only the return [`Address`](crate::types::address::Address) can.
    #[packable(verify_with = verify_timestamp)]
    timestamp: u32,
}

impl ExpirationUnlockCondition {
    /// The [`UnlockCondition`](crate::types::block::output::UnlockCondition) kind of an [`ExpirationUnlockCondition`].
    pub const KIND: u8 = 3;

    /// Creates a new [`ExpirationUnlockCondition`].
    #[inline(always)]
    pub fn new(return_address: impl Into<Address>, timestamp: u32) -> Result<Self, Error> {
        verify_timestamp::<true>(&timestamp, &())?;

        Ok(Self {
            return_address: return_address.into(),
            timestamp,
        })
    }

    /// Returns the return address of a [`ExpirationUnlockCondition`].
    #[inline(always)]
    pub fn return_address(&self) -> &Address {
        &self.return_address
    }

    /// Returns the timestamp of a [`ExpirationUnlockCondition`].
    #[inline(always)]
    pub fn timestamp(&self) -> u32 {
        self.timestamp
    }

    /// Returns the return address if the condition has expired.
    pub fn return_address_expired(&self, timestamp: u32) -> Option<&Address> {
        if timestamp >= self.timestamp() {
            Some(&self.return_address)
        } else {
            None
        }
    }
}

#[inline]
fn verify_timestamp<const VERIFY: bool>(timestamp: &u32, _: &()) -> Result<(), Error> {
    if VERIFY && *timestamp == 0 {
        Err(Error::ExpirationUnlockConditionZero)
    } else {
        Ok(())
    }
}

pub(super) mod dto {
    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::types::block::Error;

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ExpirationUnlockConditionDto {
        #[serde(rename = "type")]
        pub kind: u8,
        pub return_address: Address,
        #[serde(rename = "unixTime")]
        pub timestamp: u32,
    }

    impl From<&ExpirationUnlockCondition> for ExpirationUnlockConditionDto {
        fn from(value: &ExpirationUnlockCondition) -> Self {
            Self {
                kind: ExpirationUnlockCondition::KIND,
                return_address: *value.return_address(),
                timestamp: value.timestamp(),
            }
        }
    }

    impl TryFrom<ExpirationUnlockConditionDto> for ExpirationUnlockCondition {
        type Error = Error;

        fn try_from(value: ExpirationUnlockConditionDto) -> Result<Self, Error> {
            Self::new(value.return_address, value.timestamp)
                .map_err(|_| Error::InvalidField("expirationUnlockCondition"))
        }
    }

    impl<'de> Deserialize<'de> for ExpirationUnlockCondition {
        fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            let dto = ExpirationUnlockConditionDto::deserialize(d)?;
            if dto.kind != Self::KIND {
                return Err(serde::de::Error::custom(format!(
                    "invalid expiration unlock condition type: expected {}, found {}",
                    Self::KIND,
                    dto.kind
                )));
            }
            dto.try_into().map_err(serde::de::Error::custom)
        }
    }

    impl Serialize for ExpirationUnlockCondition {
        fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            ExpirationUnlockConditionDto::from(self).serialize(s)
        }
    }
}
