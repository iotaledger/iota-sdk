// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod block_issuance_credit;
mod commitment;
mod reward;

use core::ops::RangeInclusive;

use derive_more::{Display, From};

pub use self::{
    block_issuance_credit::BlockIssuanceCreditContextInput, commitment::CommitmentContextInput,
    reward::RewardContextInput,
};
use crate::types::block::Error;

/// The maximum number of context inputs of a transaction.
pub const CONTEXT_INPUT_COUNT_MAX: u16 = 128;
/// The range of valid numbers of context inputs of a transaction.
pub const CONTEXT_INPUT_COUNT_RANGE: RangeInclusive<u16> = 0..=CONTEXT_INPUT_COUNT_MAX; // [0..128]

/// A Context Input provides additional contextual information for the execution of a transaction, such as for different
/// functionality related to accounts, commitments, or Mana rewards. A Context Input does not need to be unlocked.
#[derive(Clone, Eq, Display, PartialEq, Hash, Ord, PartialOrd, From, packable::Packable)]
#[packable(unpack_error = Error)]
#[packable(tag_type = u8, with_error = Error::InvalidContextInputKind)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize), serde(untagged))]
pub enum ContextInput {
    /// A [`CommitmentContextInput`].
    #[packable(tag = CommitmentContextInput::KIND)]
    Commitment(CommitmentContextInput),
    /// A [`BlockIssuanceCreditContextInput`].
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

    /// Checks whether the context input is a [`CommitmentContextInput`].
    pub fn is_commitment(&self) -> bool {
        matches!(self, Self::Commitment(_))
    }

    /// Gets the input as an actual [`CommitmentContextInput`].
    /// PANIC: do not call on a non-commitment context input.
    pub fn as_commitment(&self) -> &CommitmentContextInput {
        if let Self::Commitment(input) = self {
            input
        } else {
            panic!("invalid downcast of non-CommitmentContextInput");
        }
    }

    /// Checks whether the context input is a [`BlockIssuanceCreditContextInput`].
    pub fn is_block_issuance_credit(&self) -> bool {
        matches!(self, Self::BlockIssuanceCredit(_))
    }

    /// Gets the input as an actual [`BlockIssuanceCreditContextInput`].
    /// PANIC: do not call on a non-block-issuance-credit context input.
    pub fn as_block_issuance_credit(&self) -> &BlockIssuanceCreditContextInput {
        if let Self::BlockIssuanceCredit(input) = self {
            input
        } else {
            panic!("invalid downcast of non-BlockIssuanceCreditContextInput");
        }
    }

    /// Checks whether the context input is a [`RewardContextInput`].
    pub fn is_reward(&self) -> bool {
        matches!(self, Self::Reward(_))
    }

    /// Gets the input as an actual [`RewardContextInput`].
    /// PANIC: do not call on a non-reward context input.
    pub fn as_reward(&self) -> &RewardContextInput {
        if let Self::Reward(input) = self {
            input
        } else {
            panic!("invalid downcast of non-RewardContextInput");
        }
    }
}

#[cfg(test)]
mod tests {

    use super::ContextInput;

    #[test]
    fn test_commitment() {
        let commitment: ContextInput = serde_json::from_str(
            r#"
            {
                "type": 0,
                "commitmentId": "0xedf5f572c58ddf4b4f9567d82bf96689cc68b730df796d822b4b9fb643f5efda4f9567d82bf96689"
            }
            "#,
        )
        .unwrap();
        assert!(commitment.is_commitment());
        assert_eq!(
            commitment.as_commitment().commitment_id().to_string(),
            "0xedf5f572c58ddf4b4f9567d82bf96689cc68b730df796d822b4b9fb643f5efda4f9567d82bf96689"
        );

        // Test wrong type returns error.
        let commitment_deserialization_result: Result<ContextInput, _> = serde_json::from_str(
            r#"
            {
                "type": 2,
                "commitmentId": "0xedf5f572c58ddf4b4f9567d82bf96689cc68b730df796d822b4b9fb643f5efda4f9567d82bf96689"
            }
            "#,
        );
        assert!(commitment_deserialization_result.is_err());
    }

    #[test]
    fn test_block_issuance_credit() {
        let bic: ContextInput = serde_json::from_str(
            r#"
            {
                "type": 1,
                "accountId": "0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c649"
            }
            "#,
        )
        .unwrap();
        assert!(bic.is_block_issuance_credit());
        assert_eq!(
            bic.as_block_issuance_credit().account_id().to_string(),
            "0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c649"
        );

        // Test wrong type returns error.
        let bic_deserialization_result: Result<ContextInput, _> = serde_json::from_str(
            r#"
            {
                "type": 2,
                "accountId": "0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c649"
            }
            "#,
        );
        assert!(bic_deserialization_result.is_err());
    }

    #[test]
    fn test_reward() {
        let reward: ContextInput = serde_json::from_str(
            r#"
            {
                "type": 2,
                "index": 10 
            }
            "#,
        )
        .unwrap();
        assert!(reward.is_reward());
        assert_eq!(reward.as_reward().index(), 10);

        // Test wrong type returns error.
        let reward_serialization_result: Result<ContextInput, _> = serde_json::from_str(
            r#"
            {
                "type": 0,
                "index": 10 
            }
            "#,
        );
        assert!(reward_serialization_result.is_err())
    }
}
