// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod reward;

use core::ops::RangeInclusive;

use derive_more::From;

pub use self::reward::RewardContextInput;
use crate::types::block::Error;

/// The maximum number of context inputs of a transaction.
pub const CONTEXT_INPUT_COUNT_MAX: u16 = 128;
/// The range of valid numbers of context inputs of a transaction.
pub const CONTEXT_INPUT_COUNT_RANGE: RangeInclusive<u16> = 1..=CONTEXT_INPUT_COUNT_MAX; // [1..128]

/// A Context Input provides additional contextual information for the execution of a transaction, such as for different
/// functionality related to accounts, commitments, or Mana rewards. A Context Input does not need to be unlocked.
#[derive(Clone, Eq, PartialEq, Hash, Ord, PartialOrd, From, packable::Packable)]
#[packable(unpack_error = Error)]
#[packable(tag_type = u8, with_error = Error::InvalidContextInputKind)]
pub enum ContextInput {
    /// A [`RewardContextInput`].
    #[packable(tag = RewardContextInput::KIND)]
    Reward(RewardContextInput),
    // TODO: Commitment Input https://github.com/iotaledger/iota-sdk/issues/901 and Block Issuance Credit Input https://github.com/iotaledger/iota-sdk/issues/906
}

impl core::fmt::Debug for ContextInput {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Reward(input) => input.fmt(f),
        }
    }
}

impl ContextInput {
    /// Returns the context input kind of a `ContextInput`.
    pub fn kind(&self) -> u8 {
        match self {
            Self::Reward(_) => RewardContextInput::KIND,
        }
    }

    /// Checks whether the context input is a [`RewardContextInput`].
    pub fn is_reward(&self) -> bool {
        matches!(self, Self::Reward(_))
    }

    /// Gets the input as an actual [`RewardContextInput`].
    /// PANIC: do not call on a non-reward context input.
    pub fn as_reward(&self) -> &RewardContextInput {
        let Self::Reward(input) = self;
        input
    }
}

pub mod dto {
    use serde::{Deserialize, Serialize};

    pub use super::reward::dto::RewardContextInputDto;
    use super::*;
    use crate::types::block::Error;

    /// Describes all the different context input types.
    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, From)]
    #[serde(untagged)]
    pub enum ContextInputDto {
        Reward(RewardContextInputDto),
    }

    impl From<&ContextInput> for ContextInputDto {
        fn from(value: &ContextInput) -> Self {
            match value {
                ContextInput::Reward(u) => Self::Reward(u.into()),
            }
        }
    }

    impl TryFrom<ContextInputDto> for ContextInput {
        type Error = Error;

        fn try_from(value: ContextInputDto) -> Result<Self, Self::Error> {
            match value {
                ContextInputDto::Reward(u) => Ok(Self::Reward(u.try_into()?)),
            }
        }
    }
}
