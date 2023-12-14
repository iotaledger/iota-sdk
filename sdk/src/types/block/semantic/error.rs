// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt;

use crate::types::block::Error;

/// Describes the reason of a transaction failure.
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, packable::Packable)]
#[cfg_attr(feature = "serde", derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr))]
#[packable(unpack_error = Error)]
#[packable(tag_type = u8, with_error = Error::InvalidTransactionFailureReason)]
#[non_exhaustive]
pub enum TransactionFailureReason {
    /// The referenced UTXO was already spent.
    InputUtxoAlreadySpent = 1,
    /// The transaction is conflicting with another transaction. Conflicting specifically means a double spend
    /// situation that both transaction pass all validation rules, eventually losing one(s) should have this reason.
    ConflictingWithAnotherTx = 2,
    /// The referenced UTXO is invalid.
    InvalidReferencedUtxo = 3,
    /// The transaction is invalid.
    InvalidTransaction = 4,
    /// The sum of the inputs and output base token amount does not match.
    SumInputsOutputsAmountMismatch = 5,
    /// The unlock block signature is invalid.
    InvalidUnlockBlockSignature = 6,
    /// The configured timelock is not yet expired.
    TimelockNotExpired = 7,
    /// The given native tokens are invalid.
    InvalidNativeTokens = 8,
    /// The return amount in a transaction is not fulfilled by the output side.
    StorageDepositReturnUnfulfilled = 9,
    /// An input unlock was invalid.
    InvalidInputUnlock = 10,
    /// The output contains a Sender with an ident (address) which is not unlocked.
    SenderNotUnlocked = 11,
    /// The chain state transition is invalid.
    InvalidChainStateTransition = 12,
    /// The referenced input is created after transaction issuing time.
    InvalidTransactionIssuingTime = 13,
    /// The mana amount is invalid.
    InvalidManaAmount = 14,
    /// The Block Issuance Credits amount is invalid.
    InvalidBlockIssuanceCreditsAmount = 15,
    /// Reward Context Input is invalid.
    InvalidRewardContextInput = 16,
    /// Commitment Context Input is invalid.
    InvalidCommitmentContextInput = 17,
    /// Staking Feature is not provided in account output when claiming rewards.
    MissingStakingFeature = 18,
    /// Failed to claim staking reward.
    FailedToClaimStakingReward = 19,
    /// Failed to claim delegation reward.
    FailedToClaimDelegationReward = 20,
    /// Burning of native tokens is not allowed in the transaction capabilities.
    TransactionCapabilityNativeTokenBurningNotAllowed = 21,
    /// Burning of mana is not allowed in the transaction capabilities.
    TransactionCapabilityManaBurningNotAllowed = 22,
    /// Destruction of accounts is not allowed in the transaction capabilities.
    TransactionCapabilityAccountDestructionNotAllowed = 23,
    /// Destruction of anchors is not allowed in the transaction capabilities.
    TransactionCapabilityAnchorDestructionNotAllowed = 24,
    /// Destruction of foundries is not allowed in the transaction capabilities.
    TransactionCapabilityFoundryDestructionNotAllowed = 25,
    /// Destruction of nfts is not allowed in the transaction capabilities.
    TransactionCapabilityNftDestructionNotAllowed = 26,
    /// The semantic validation failed for a reason not covered by the previous variants.
    SemanticValidationFailed = 255,
}

impl fmt::Display for TransactionFailureReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InputUtxoAlreadySpent => write!(f, "The referenced UTXO was already spent."),
            Self::ConflictingWithAnotherTx => write!(
                f,
                "The transaction is conflicting with another transaction. Conflicting specifically means a double spend situation that both transactions pass all validation rules, eventually losing one(s) should have this reason."
            ),
            Self::InvalidReferencedUtxo => write!(f, "The referenced UTXO is invalid."),
            Self::InvalidTransaction => write!(f, "The transaction is invalid."),
            Self::SumInputsOutputsAmountMismatch => {
                write!(f, "The sum of the inputs and output base token amount does not match.")
            }
            Self::InvalidUnlockBlockSignature => write!(f, "The unlock block signature is invalid."),
            Self::TimelockNotExpired => write!(f, "The configured timelock is not yet expired."),
            Self::InvalidNativeTokens => write!(f, "The given native tokens are invalid."),
            Self::StorageDepositReturnUnfulfilled => write!(
                f,
                "The return amount in a transaction is not fulfilled by the output side."
            ),
            Self::InvalidInputUnlock => write!(f, "An input unlock was invalid."),
            Self::SenderNotUnlocked => write!(
                f,
                "The output contains a Sender with an ident (address) which is not unlocked."
            ),
            Self::InvalidChainStateTransition => write!(f, "The chain state transition is invalid."),
            Self::InvalidTransactionIssuingTime => {
                write!(f, "The referenced input is created after transaction issuing time.")
            }
            Self::InvalidManaAmount => write!(f, "The mana amount is invalid."),
            Self::InvalidBlockIssuanceCreditsAmount => write!(f, "The Block Issuance Credits amount is invalid."),
            Self::InvalidRewardContextInput => write!(f, "Reward Context Input is invalid."),
            Self::InvalidCommitmentContextInput => write!(f, "Commitment Context Input is invalid."),
            Self::MissingStakingFeature => write!(
                f,
                "Staking Feature is not provided in account output when claiming rewards."
            ),
            Self::FailedToClaimStakingReward => write!(f, "Failed to claim staking reward."),
            Self::FailedToClaimDelegationReward => write!(f, "Failed to claim delegation reward."),
            Self::TransactionCapabilityNativeTokenBurningNotAllowed => write!(
                f,
                "Burning of native tokens is not allowed in the transaction capabilities."
            ),
            Self::TransactionCapabilityManaBurningNotAllowed => {
                write!(f, "Burning of mana is not allowed in the transaction capabilities.")
            }
            Self::TransactionCapabilityAccountDestructionNotAllowed => write!(
                f,
                "Destruction of accounts is not allowed in the transaction capabilities."
            ),
            Self::TransactionCapabilityAnchorDestructionNotAllowed => write!(
                f,
                "Destruction of anchors is not allowed in the transaction capabilities."
            ),
            Self::TransactionCapabilityFoundryDestructionNotAllowed => write!(
                f,
                "Destruction of foundries is not allowed in the transaction capabilities."
            ),
            Self::TransactionCapabilityNftDestructionNotAllowed => {
                write!(f, "Destruction of nfts is not allowed in the transaction capabilities.")
            }
            Self::SemanticValidationFailed => write!(
                f,
                "The semantic validation failed for a reason not covered by the previous variants."
            ),
        }
    }
}

impl TryFrom<u8> for TransactionFailureReason {
    type Error = Error;

    fn try_from(c: u8) -> Result<Self, Self::Error> {
        Ok(match c {
            1 => Self::InputUtxoAlreadySpent,
            2 => Self::ConflictingWithAnotherTx,
            3 => Self::InvalidReferencedUtxo,
            4 => Self::InvalidTransaction,
            5 => Self::SumInputsOutputsAmountMismatch,
            6 => Self::InvalidUnlockBlockSignature,
            7 => Self::TimelockNotExpired,
            8 => Self::InvalidNativeTokens,
            9 => Self::StorageDepositReturnUnfulfilled,
            10 => Self::InvalidInputUnlock,
            11 => Self::SenderNotUnlocked,
            12 => Self::InvalidChainStateTransition,
            13 => Self::InvalidTransactionIssuingTime,
            14 => Self::InvalidManaAmount,
            15 => Self::InvalidBlockIssuanceCreditsAmount,
            16 => Self::InvalidRewardContextInput,
            17 => Self::InvalidCommitmentContextInput,
            18 => Self::MissingStakingFeature,
            19 => Self::FailedToClaimStakingReward,
            20 => Self::FailedToClaimDelegationReward,
            21 => Self::TransactionCapabilityNativeTokenBurningNotAllowed,
            22 => Self::TransactionCapabilityManaBurningNotAllowed,
            23 => Self::TransactionCapabilityAccountDestructionNotAllowed,
            24 => Self::TransactionCapabilityAnchorDestructionNotAllowed,
            25 => Self::TransactionCapabilityFoundryDestructionNotAllowed,
            26 => Self::TransactionCapabilityNftDestructionNotAllowed,
            255 => Self::SemanticValidationFailed,
            x => return Err(Self::Error::InvalidTransactionFailureReason(x)),
        })
    }
}
