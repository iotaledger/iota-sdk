// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod block_issuance_credit;
mod commitment;
mod reward;

pub use self::block_issuance_credit::BlockIssuanceCreditContextInput;
pub use self::commitment::CommitmentContextInput;
pub use self::reward::RewardContextInput;

use crate::types::block::Error;
use derive_more::Display;
use derive_more::From;

/// A Context Input provides additional contextual information for the execution of a transaction, such as for different
/// functionality related to accounts, commitments, or Mana rewards. A Context Input does not need to be unlocked.
#[derive(Clone, Eq, Display, PartialEq, Hash, Ord, PartialOrd, From, packable::Packable)]
#[packable(unpack_error = Error)]
#[packable(tag_type = u8, with_error = Error::InvalidContextInputKind)]
pub enum ContextInput {
    /// A [`CommitmentContextInput`].
    #[packable(tag = CommitmentContextInput::KIND)]
    Commitment(CommitmentContextInput),

    /// A [`CommitmentContextInput`].
    #[packable(tag = BlockIssuanceCreditContextInput::KIND)]
    BlockIssuanceCredit(BlockIssuanceCreditContextInput),

    /// A [`RewardContextInput`].
    #[packable(tag = RewardContextInput::KIND)]
    Reward(RewardContextInput),
}

impl core::fmt::Debug for ContextInput {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Commitment(input) => input.fmt(f),
            Self::BlockIssuanceCredit(input) => input.fmt(f),
            Self::Reward(input) => input.fmt(f),
        }
    }
}

impl ContextInput {
    /// Returns the context input kind of a `ContextInput`.
    pub fn kind(&self) -> u8 {
        match self {
            Self::Commitment(_) => CommitmentContextInput::KIND,
            Self::BlockIssuanceCredit(_) => BlockIssuanceCreditContextInput::KIND,
            Self::Reward(_) => RewardContextInput::KIND,
        }
    }

    /// Checks whether the context input is a [`RewardContextInput`].
    pub fn is_reward(&self) -> bool {
        matches!(self, Self::Reward(_))
    }

    /// Checks whether the context input is a [`CommitmentContextInput`].
    pub fn is_commitment(&self) -> bool {
        matches!(self, Self::Commitment(_))
    }

    /// Checks whether the context input is a [`BlockIssuanceCreditContextInput`].
    pub fn is_block_issuance_credit(&self) -> bool {
        matches!(self, Self::BlockIssuanceCredit(_))
    }

    /// Gets the input as an actual [`RewardContextInput`].
    /// PANIC: do not call on a non-reward context input.
    pub fn as_reward(&self) -> &RewardContextInput {
        if let Self::Reward(input) = self {
            input
        } else {
            panic!("context input is not of type reward: {:?}", self);
        }
    }

    /// Gets the input as an actual [`CommitmentContextInput`].
    /// PANIC: do not call on a non-commitment context input.
    pub fn as_commitment(&self) -> &CommitmentContextInput {
        if let Self::Commitment(input) = self {
            input
        } else {
            panic!("context input is not of type commitment: {:?}", self);
        }
    }
    /// Gets the input as an actual [`BlockIssuanceCreditContextInput`].
    /// PANIC: do not call on a non-block-issuance-credit context input.
    pub fn as_block_issuance_credit(&self) -> &BlockIssuanceCreditContextInput {
        if let Self::BlockIssuanceCredit(input) = self {
            input
        } else {
            panic!("context input is not of type block issuance credit: {:?}", self);
        }
    }
}

pub mod dto {
    use serde::{Deserialize, Serialize};

    pub use super::reward::dto::RewardContextInputDto;
    use super::{
        block_issuance_credit::dto::BlockIssuanceCreditContextInputDto, commitment::dto::CommitmentContextInputDto, *,
    };
    use crate::types::block::Error;

    /// Describes all the different context input types.
    #[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, From)]
    #[serde(untagged)]
    pub enum ContextInputDto {
        Commitment(CommitmentContextInputDto),
        BlockIssuanceCredit(BlockIssuanceCreditContextInputDto),
        Reward(RewardContextInputDto),
    }

    impl From<&ContextInput> for ContextInputDto {
        fn from(value: &ContextInput) -> Self {
            match value {
                ContextInput::Commitment(u) => Self::Commitment(u.into()),
                ContextInput::BlockIssuanceCredit(u) => Self::BlockIssuanceCredit(u.into()),
                ContextInput::Reward(u) => Self::Reward(u.into()),
            }
        }
    }

    impl TryFrom<ContextInputDto> for ContextInput {
        type Error = Error;

        fn try_from(value: ContextInputDto) -> Result<Self, Self::Error> {
            match value {
                ContextInputDto::Commitment(u) => Ok(Self::Commitment(u.try_into()?)),
                ContextInputDto::BlockIssuanceCredit(u) => Ok(Self::BlockIssuanceCredit(u.try_into()?)),
                ContextInputDto::Reward(u) => Ok(Self::Reward(u.try_into()?)),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{CommitmentContextInput, ContextInput, RewardContextInput};
    use crate::types::block::{
        context_input::BlockIssuanceCreditContextInput, output::AccountId, slot::SlotCommitmentId,
    };
    use core::str::FromStr;

    #[test]
    fn test_context_input() {
        let reward = ContextInput::Reward(RewardContextInput::new(10));
        let reward: &RewardContextInput = reward.as_reward();
        assert_eq!(reward.to_string(), "10");

        let slot_commitment_id_str =
            "0xedf5f572c58ddf4b4f9567d82bf96689cc68b730df796d822b4b9fb643f5efda4f9567d82bf96689";
        let slot_commitment_id = SlotCommitmentId::from_str(slot_commitment_id_str).unwrap();
        let commitment = ContextInput::Commitment(CommitmentContextInput::new(slot_commitment_id));
        let commitment: &CommitmentContextInput = commitment.as_commitment();
        assert_eq!(commitment.to_string(), slot_commitment_id_str);

        let account_id_str = "0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c649";
        let account_id = AccountId::from_str(account_id_str).unwrap();
        let block_issuance_credit = ContextInput::BlockIssuanceCredit(BlockIssuanceCreditContextInput::new(account_id));
        let block_issuance_credit: &BlockIssuanceCreditContextInput = block_issuance_credit.as_block_issuance_credit();
        assert_eq!(block_issuance_credit.to_string(), account_id_str);
    }
}
