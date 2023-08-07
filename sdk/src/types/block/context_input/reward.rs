// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use derive_more::{Display, From};

/// A Reward Context Input indicates which transaction Input is claiming Mana rewards.
#[derive(Clone, Copy, Debug, Display, Eq, PartialEq, Hash, Ord, PartialOrd, From, packable::Packable)]
pub struct RewardContextInput(u16);

impl RewardContextInput {
    /// The context input kind of a [`RewardContextInput`].
    pub const KIND: u8 = 2;

    /// Creates a new [`RewardContextInput`].
    pub fn new(index: u16) -> Self {
        Self(index)
    }

    /// Returns the index of a [`RewardContextInput`].
    pub fn index(&self) -> u16 {
        self.0
    }
}

mod dto {
    use serde::{Deserialize, Serialize};

    use super::*;

    /// A Reward Context Input is an input that indicates which transaction Input is claiming Mana rewards.
    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
    struct RewardContextInputDto {
        #[serde(rename = "type")]
        kind: u8,
        index: u16,
    }

    impl From<&RewardContextInput> for RewardContextInputDto {
        fn from(value: &RewardContextInput) -> Self {
            Self {
                kind: RewardContextInput::KIND,
                index: value.index(),
            }
        }
    }

    impl From<RewardContextInputDto> for RewardContextInput {
        fn from(value: RewardContextInputDto) -> Self {
            Self::new(value.index)
        }
    }

    impl_serde_typed_dto!(RewardContextInput, RewardContextInputDto, "reward context input");
}
