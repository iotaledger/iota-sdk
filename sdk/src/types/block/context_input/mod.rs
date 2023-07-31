// Copyright 2020-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod reward;

use derive_more::From;

use self::reward::RewardInput;
use crate::types::block::Error;

/// A generic input supporting different input kinds.
#[derive(Clone, Eq, PartialEq, Hash, Ord, PartialOrd, From, packable::Packable)]
#[packable(unpack_error = Error)]
#[packable(tag_type = u8, with_error = Error::InvalidInputKind)]
pub enum ContextInput {
    /// A reward input.
    #[packable(tag = RewardInput::KIND)]
    Reward(RewardInput),
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
    /// Returns the input kind of an `ContextInput`.
    pub fn kind(&self) -> u8 {
        match self {
            Self::Reward(_) => RewardInput::KIND,
        }
    }

    /// Checks whether the input is a [`RewardInput`].
    pub fn is_utxo(&self) -> bool {
        matches!(self, Self::Reward(_))
    }

    /// Gets the input as an actual [`RewardInput`].
    /// PANIC: do not call on a non-reward input.
    pub fn as_reward(&self) -> &RewardInput {
        let Self::Reward(input) = self;
        input
    }
}

pub mod dto {
    use serde::{Deserialize, Serialize};

    pub use super::reward::dto::RewardInputDto;
    use super::*;
    use crate::types::block::Error;

    /// Describes all the different context input types.
    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, From)]
    #[serde(untagged)]
    pub enum ContextInputDto {
        Reward(RewardInputDto),
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
