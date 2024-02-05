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
    None = 0,
    TypeInvalid = 1,
    Conflicting = 2,
    InputAlreadySpent = 3,
    InputCreationAfterTxCreation = 4,
    UnlockSignatureInvalid = 5,
    CommitmentInputMissing = 6,
    CommitmentInputReferenceInvalid = 7,
    BicInputReferenceInvalid = 8,
    RewardInputReferenceInvalid = 9,
    StakingRewardCalculationFailure = 10,
    DelegationRewardCalculationFailure = 11,
    InputOutputBaseTokenMismatch = 12,
    ManaOverflow = 13,
    InputOutputManaMismatch = 14,
    ManaDecayCreationIndexExceedsTargetIndex = 15,
    NativeTokenAmountLessThanZero = 16,
    NativeTokenSumExceedsUint256 = 17,
    NativeTokenSumUnbalanced = 18,
    MultiAddressLengthUnlockLengthMismatch = 19,
    MultiAddressUnlockThresholdNotReached = 20,
    NestedMultiUnlock = 21,
    SenderFeatureNotUnlocked = 22,
    IssuerFeatureNotUnlocked = 23,
    StakingRewardInputMissing = 24,
    StakingBlockIssuerFeatureMissing = 25,
    StakingCommitmentInputMissing = 26,
    StakingRewardClaimingInvalid = 27,
    StakingFeatureRemovedBeforeUnbonding = 28,
    StakingFeatureModifiedBeforeUnbonding = 29,
    StakingStartEpochInvalid = 30,
    StakingEndEpochTooEarly = 31,
    BlockIssuerCommitmentInputMissing = 32,
    BlockIssuanceCreditInputMissing = 33,
    BlockIssuerNotExpired = 34,
    BlockIssuerExpiryTooEarly = 35,
    ManaMovedOffBlockIssuerAccount = 36,
    AccountLocked = 37,
    TimelockCommitmentInputMissing = 38,
    TimelockNotExpired = 39,
    ExpirationCommitmentInputMissing = 40,
    ExpirationNotUnlockable = 41,
    ReturnAmountNotFulFilled = 42,
    NewChainOutputHasNonZeroedId = 43,
    ChainOutputImmutableFeaturesChanged = 44,
    ImplicitAccountDestructionDisallowed = 45,
    MultipleImplicitAccountCreationAddresses = 46,
    AccountInvalidFoundryCounter = 47,
    FoundryTransitionWithoutAccount = 48,
    FoundrySerialInvalid = 49,
    DelegationCommitmentInputMissing = 50,
    DelegationRewardInputMissing = 51,
    DelegationRewardsClaimingInvalid = 52,
    DelegationOutputTransitionedTwice = 53,
    DelegationModified = 54,
    DelegationStartEpochInvalid = 55,
    DelegationAmountMismatch = 56,
    DelegationEndEpochNotZero = 57,
    DelegationEndEpochInvalid = 58,
    CapabilitiesNativeTokenBurningNotAllowed = 59,
    CapabilitiesManaBurningNotAllowed = 60,
    CapabilitiesAccountDestructionNotAllowed = 61,
    CapabilitiesAnchorDestructionNotAllowed = 62,
    CapabilitiesFoundryDestructionNotAllowed = 63,
    CapabilitiesNftDestructionNotAllowed = 64,
    SemanticValidationFailed = 255,
}

// impl fmt::Display for TransactionFailureReason {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Self::InputUtxoAlreadySpent => write!(f, "The referenced UTXO was already spent."),
//             Self::ConflictingWithAnotherTx => write!(
//                 f,
//                 "The transaction is conflicting with another transaction. Conflicting specifically means a double spend situation that both transactions pass all validation rules, eventually losing one(s) should have this reason."
//             ),
//             Self::InvalidReferencedUtxo => write!(f, "The referenced UTXO is invalid."),
//             Self::InvalidTransaction => write!(f, "The transaction is invalid."),
//             Self::SumInputsOutputsAmountMismatch => {
//                 write!(f, "The sum of the inputs and output base token amount does not match.")
//             }
//             Self::InvalidUnlockBlockSignature => write!(f, "The unlock block signature is invalid."),
//             Self::TimelockNotExpired => write!(f, "The configured timelock is not yet expired."),
//             Self::InvalidNativeTokens => write!(f, "The given native tokens are invalid."),
//             Self::StorageDepositReturnUnfulfilled => write!(
//                 f,
//                 "The return amount in a transaction is not fulfilled by the output side."
//             ),
//             Self::InvalidInputUnlock => write!(f, "An input unlock was invalid."),
//             Self::SenderNotUnlocked => write!(
//                 f,
//                 "The output contains a Sender with an ident (address) which is not unlocked."
//             ),
//             Self::InvalidChainStateTransition => write!(f, "The chain state transition is invalid."),
//             Self::InvalidTransactionIssuingTime => {
//                 write!(f, "The referenced input is created after transaction issuing time.")
//             }
//             Self::InvalidManaAmount => write!(f, "The mana amount is invalid."),
//             Self::InvalidBlockIssuanceCreditsAmount => write!(f, "The Block Issuance Credits amount is invalid."),
//             Self::InvalidRewardContextInput => write!(f, "Reward Context Input is invalid."),
//             Self::InvalidCommitmentContextInput => write!(f, "Commitment Context Input is invalid."),
//             Self::MissingStakingFeature => write!(
//                 f,
//                 "Staking Feature is not provided in account output when claiming rewards."
//             ),
//             Self::FailedToClaimStakingReward => write!(f, "Failed to claim staking reward."),
//             Self::FailedToClaimDelegationReward => write!(f, "Failed to claim delegation reward."),
//             Self::TransactionCapabilityNativeTokenBurningNotAllowed => write!(
//                 f,
//                 "Burning of native tokens is not allowed in the transaction capabilities."
//             ),
//             Self::TransactionCapabilityManaBurningNotAllowed => {
//                 write!(f, "Burning of mana is not allowed in the transaction capabilities.")
//             }
//             Self::TransactionCapabilityAccountDestructionNotAllowed => write!(
//                 f,
//                 "Destruction of accounts is not allowed in the transaction capabilities."
//             ),
//             Self::TransactionCapabilityAnchorDestructionNotAllowed => write!(
//                 f,
//                 "Destruction of anchors is not allowed in the transaction capabilities."
//             ),
//             Self::TransactionCapabilityFoundryDestructionNotAllowed => write!(
//                 f,
//                 "Destruction of foundries is not allowed in the transaction capabilities."
//             ),
//             Self::TransactionCapabilityNftDestructionNotAllowed => {
//                 write!(f, "Destruction of nfts is not allowed in the transaction capabilities.")
//             }
//             Self::SemanticValidationFailed => write!(
//                 f,
//                 "The semantic validation failed for a reason not covered by the previous variants."
//             ),
//         }
//     }
// }

impl TryFrom<u8> for TransactionFailureReason {
    type Error = Error;

    fn try_from(c: u8) -> Result<Self, Self::Error> {
        Ok(match c {
            0 => Self::None,
            1 => Self::TypeInvalid,
            2 => Self::Conflicting,
            3 => Self::InputAlreadySpent,
            4 => Self::InputCreationAfterTxCreation,
            5 => Self::UnlockSignatureInvalid,
            6 => Self::CommitmentInputMissing,
            7 => Self::CommitmentInputReferenceInvalid,
            8 => Self::BicInputReferenceInvalid,
            9 => Self::RewardInputReferenceInvalid,
            10 => Self::StakingRewardCalculationFailure,
            11 => Self::DelegationRewardCalculationFailure,
            12 => Self::InputOutputBaseTokenMismatch,
            13 => Self::ManaOverflow,
            14 => Self::InputOutputManaMismatch,
            15 => Self::ManaDecayCreationIndexExceedsTargetIndex,
            16 => Self::NativeTokenAmountLessThanZero,
            17 => Self::NativeTokenSumExceedsUint256,
            18 => Self::NativeTokenSumUnbalanced,
            19 => Self::MultiAddressLengthUnlockLengthMismatch,
            20 => Self::MultiAddressUnlockThresholdNotReached,
            21 => Self::NestedMultiUnlock,
            22 => Self::SenderFeatureNotUnlocked,
            23 => Self::IssuerFeatureNotUnlocked,
            24 => Self::StakingRewardInputMissing,
            25 => Self::StakingBlockIssuerFeatureMissing,
            26 => Self::StakingCommitmentInputMissing,
            27 => Self::StakingRewardClaimingInvalid,
            28 => Self::StakingFeatureRemovedBeforeUnbonding,
            29 => Self::StakingFeatureModifiedBeforeUnbonding,
            30 => Self::StakingStartEpochInvalid,
            31 => Self::StakingEndEpochTooEarly,
            32 => Self::BlockIssuerCommitmentInputMissing,
            33 => Self::BlockIssuanceCreditInputMissing,
            34 => Self::BlockIssuerNotExpired,
            35 => Self::BlockIssuerExpiryTooEarly,
            36 => Self::ManaMovedOffBlockIssuerAccount,
            37 => Self::AccountLocked,
            38 => Self::TimelockCommitmentInputMissing,
            39 => Self::TimelockNotExpired,
            40 => Self::ExpirationCommitmentInputMissing,
            41 => Self::ExpirationNotUnlockable,
            42 => Self::ReturnAmountNotFulFilled,
            43 => Self::NewChainOutputHasNonZeroedId,
            44 => Self::ChainOutputImmutableFeaturesChanged,
            45 => Self::ImplicitAccountDestructionDisallowed,
            46 => Self::MultipleImplicitAccountCreationAddresses,
            47 => Self::AccountInvalidFoundryCounter,
            48 => Self::FoundryTransitionWithoutAccount,
            49 => Self::FoundrySerialInvalid,
            50 => Self::DelegationCommitmentInputMissing,
            51 => Self::DelegationRewardInputMissing,
            52 => Self::DelegationRewardsClaimingInvalid,
            53 => Self::DelegationOutputTransitionedTwice,
            54 => Self::DelegationModified,
            55 => Self::DelegationStartEpochInvalid,
            56 => Self::DelegationAmountMismatch,
            57 => Self::DelegationEndEpochNotZero,
            58 => Self::DelegationEndEpochInvalid,
            59 => Self::CapabilitiesNativeTokenBurningNotAllowed,
            60 => Self::CapabilitiesManaBurningNotAllowed,
            61 => Self::CapabilitiesAccountDestructionNotAllowed,
            62 => Self::CapabilitiesAnchorDestructionNotAllowed,
            63 => Self::CapabilitiesFoundryDestructionNotAllowed,
            64 => Self::CapabilitiesNftDestructionNotAllowed,
            255 => Self::SemanticValidationFailed,
            x => return Err(Self::Error::InvalidTransactionFailureReason(x)),
        })
    }
}
