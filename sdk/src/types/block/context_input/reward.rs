// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use packable::bounded::BoundedU16;

use super::CONTEXT_INPUT_COUNT_RANGE;
use crate::types::block::{
    context_input::ContextInputError,
    protocol::{WorkScore, WorkScoreParameters},
};

pub(crate) type RewardContextInputIndex =
    BoundedU16<{ *CONTEXT_INPUT_COUNT_RANGE.start() }, { *CONTEXT_INPUT_COUNT_RANGE.end() }>;

/// A Reward Context Input indicates which transaction Input is claiming Mana rewards.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, packable::Packable)]
#[packable(unpack_error = ContextInputError)]
pub struct RewardContextInput(#[packable(unpack_error_with = ContextInputError::RewardIndex)] RewardContextInputIndex);

impl RewardContextInput {
    /// The context input kind of a [`RewardContextInput`].
    pub const KIND: u8 = 2;

    /// Creates a new [`RewardContextInput`].
    pub fn new(index: u16) -> Result<Self, ContextInputError> {
        index.try_into().map(Self).map_err(ContextInputError::RewardIndex)
    }

    /// Returns the index of a [`RewardContextInput`].
    pub fn index(&self) -> u16 {
        self.0.get()
    }
}

impl WorkScore for RewardContextInput {
    fn work_score(&self, params: WorkScoreParameters) -> u32 {
        params.context_input()
    }
}

impl core::fmt::Display for RewardContextInput {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "RewardContextInput({})", self.index())
    }
}

#[cfg(feature = "serde")]
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

    impl TryFrom<RewardContextInputDto> for RewardContextInput {
        type Error = ContextInputError;

        fn try_from(value: RewardContextInputDto) -> Result<Self, Self::Error> {
            Self::new(value.index)
        }
    }

    crate::impl_serde_typed_dto!(RewardContextInput, RewardContextInputDto, "reward context input");
}
