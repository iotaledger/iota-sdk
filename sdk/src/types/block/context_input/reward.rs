// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::str::FromStr;

use derive_more::From;

use crate::types::block::Error;

/// A Reward Input is an input that indicates which transaction Input is claiming Mana rewards.
#[derive(Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd, From, packable::Packable)]
pub struct RewardContextInput(u16);

impl RewardContextInput {
    /// The context input kind of a [`RewardInput`].
    pub const KIND: u8 = 2;

    /// Creates a new [`RewardInput`].
    pub fn new(index: u16) -> Self {
        Self(index)
    }

    /// Returns the index of a [`RewardInput`].
    pub fn index(&self) -> u16 {
        self.0
    }
}

#[cfg(feature = "serde")]
string_serde_impl!(RewardContextInput);

impl FromStr for RewardContextInput {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(u16::from_str(s)?))
    }
}

impl core::fmt::Display for RewardContextInput {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl core::fmt::Debug for RewardContextInput {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "RewardInput({})", self.0)
    }
}

pub(crate) mod dto {
    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::types::block::Error;

    /// A Reward Input is an input that indicates which transaction Input is claiming Mana rewards.
    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    pub struct RewardContextInputDto {
        #[serde(rename = "type")]
        pub kind: u8,
        pub index: u16,
    }

    impl From<&RewardContextInput> for RewardContextInputDto {
        fn from(value: &RewardContextInput) -> Self {
            Self {
                kind: RewardContextInput::KIND,
                index: value.index(),
            }
        }
    }

    impl TryFrom<RewardContextInputDto> for RewardContextInput {
        type Error = Error;

        fn try_from(value: RewardContextInputDto) -> Result<Self, Self::Error> {
            Ok(Self::new(value.index))
        }
    }
}
