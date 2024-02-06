// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt;

use crate::types::block::Error;

/// Describes the reason of a transaction failure.
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, packable::Packable, strum::FromRepr)]
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
    // TODO syntactic? https://github.com/iotaledger/iota-sdk/issues/1954
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
    // TODO remove? https://github.com/iotaledger/iota-sdk/issues/1954
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

impl fmt::Display for TransactionFailureReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "none."),
            Self::TypeInvalid => write!(f, "transaction type is invalid."),
            Self::Conflicting => write!(f, "transaction is conflicting."),
            Self::InputAlreadySpent => write!(f, "input already spent."),
            Self::InputCreationAfterTxCreation => write!(f, "input creation slot after tx creation slot."),
            Self::UnlockSignatureInvalid => write!(f, "signature in unlock is invalid."),
            Self::CommitmentInputMissing => write!(f, "commitment input required with reward or BIC input."),
            Self::CommitmentInputReferenceInvalid => {
                write!(f, "commitment input references an invalid or non-existent commitment.")
            }
            Self::BicInputReferenceInvalid => write!(f, "BIC input reference cannot be loaded."),
            Self::RewardInputReferenceInvalid => write!(
                f,
                "reward input does not reference a staking account or a delegation output."
            ),
            Self::StakingRewardCalculationFailure => write!(
                f,
                "staking rewards could not be calculated due to storage issues or overflow."
            ),
            Self::DelegationRewardCalculationFailure => write!(
                f,
                "delegation rewards could not be calculated due to storage issues or overflow."
            ),
            Self::InputOutputBaseTokenMismatch => write!(
                f,
                "inputs and outputs do not spend/deposit the same amount of base tokens."
            ),
            Self::ManaOverflow => write!(f, "under- or overflow in Mana calculations."),
            Self::InputOutputManaMismatch => write!(f, "inputs and outputs do not contain the same amount of Mana."),
            Self::ManaDecayCreationIndexExceedsTargetIndex => write!(
                f,
                "mana decay creation slot/epoch index exceeds target slot/epoch index."
            ),
            Self::NativeTokenAmountLessThanZero => write!(f, "native token amount must be greater than zero."),
            Self::NativeTokenSumExceedsUint256 => write!(f, "native token sum exceeds max value of a uint256."),
            Self::NativeTokenSumUnbalanced => write!(f, "native token sums are unbalanced."),
            Self::MultiAddressLengthUnlockLengthMismatch => {
                write!(f, "multi address length and multi unlock length do not match.")
            }
            Self::MultiAddressUnlockThresholdNotReached => write!(f, "multi address unlock threshold not reached."),
            Self::NestedMultiUnlock => write!(f, "multi unlocks can't be nested."),
            Self::SenderFeatureNotUnlocked => write!(f, "sender feature is not unlocked."),
            Self::IssuerFeatureNotUnlocked => write!(f, "issuer feature is not unlocked."),
            Self::StakingRewardInputMissing => {
                write!(f, "staking feature removal or resetting requires a reward input.")
            }
            Self::StakingBlockIssuerFeatureMissing => {
                write!(f, "block issuer feature missing for account with staking feature.")
            }
            Self::StakingCommitmentInputMissing => write!(f, "staking feature validation requires a commitment input."),
            Self::StakingRewardClaimingInvalid => {
                write!(f, "staking feature must be removed or reset in order to claim rewards.")
            }
            Self::StakingFeatureRemovedBeforeUnbonding => {
                write!(f, "staking feature can only be removed after the unbonding period.")
            }
            Self::StakingFeatureModifiedBeforeUnbonding => write!(
                f,
                "staking start epoch, fixed cost and staked amount cannot be modified while bonded."
            ),
            Self::StakingStartEpochInvalid => write!(f, "staking start epoch must be the epoch of the transaction."),
            Self::StakingEndEpochTooEarly => write!(
                f,
                "staking end epoch must be set to the transaction epoch plus the unbonding period."
            ),
            Self::BlockIssuerCommitmentInputMissing => write!(f, "commitment input missing for block issuer feature."),
            Self::BlockIssuanceCreditInputMissing => write!(
                f,
                "block issuance credit input missing for account with block issuer feature."
            ),
            Self::BlockIssuerNotExpired => write!(f, "block issuer feature has not expired."),
            Self::BlockIssuerExpiryTooEarly => write!(f, "block issuer feature expiry set too early."),
            Self::ManaMovedOffBlockIssuerAccount => write!(
                f,
                "mana cannot be moved off block issuer accounts except with manalocks."
            ),
            Self::AccountLocked => write!(f, "account is locked due to negative block issuance credits."),
            Self::TimelockCommitmentInputMissing => write!(
                f,
                "transaction's containing a timelock condition require a commitment input."
            ),
            Self::TimelockNotExpired => write!(f, "timelock not expired."),
            Self::ExpirationCommitmentInputMissing => write!(
                f,
                "transaction's containing an expiration condition require a commitment input."
            ),
            Self::ExpirationNotUnlockable => write!(f, "expiration unlock condition cannot be unlocked."),
            Self::ReturnAmountNotFulFilled => write!(f, "return amount not fulfilled."),
            Self::NewChainOutputHasNonZeroedId => write!(f, "new chain output has non-zeroed ID."),
            Self::ChainOutputImmutableFeaturesChanged => {
                write!(f, "immutable features in chain output modified during transition.")
            }
            Self::ImplicitAccountDestructionDisallowed => {
                write!(f, "cannot destroy implicit account; must be transitioned to account.")
            }
            Self::MultipleImplicitAccountCreationAddresses => {
                write!(f, "multiple implicit account creation addresses on the input side.")
            }
            Self::AccountInvalidFoundryCounter => write!(
                f,
                "foundry counter in account decreased or did not increase by the number of new foundries."
            ),
            Self::FoundryTransitionWithoutAccount => write!(
                f,
                "foundry output transitioned without accompanying account on input or output side."
            ),
            Self::FoundrySerialInvalid => write!(f, "foundry output serial number is invalid."),
            Self::DelegationCommitmentInputMissing => {
                write!(f, "delegation output validation requires a commitment input.")
            }
            Self::DelegationRewardInputMissing => {
                write!(f, "delegation output cannot be destroyed without a reward input.")
            }
            Self::DelegationRewardsClaimingInvalid => write!(f, "invalid delegation mana rewards claiming."),
            Self::DelegationOutputTransitionedTwice => {
                write!(f, "delegation output attempted to be transitioned twice.")
            }
            Self::DelegationModified => write!(f, "delegated amount, validator ID and start epoch cannot be modified."),
            Self::DelegationStartEpochInvalid => write!(f, "invalid start epoch."),
            Self::DelegationAmountMismatch => write!(f, "delegated amount does not match amount."),
            Self::DelegationEndEpochNotZero => write!(f, "end epoch must be set to zero at output genesis."),
            Self::DelegationEndEpochInvalid => write!(f, "delegation end epoch does not match current epoch."),
            Self::CapabilitiesNativeTokenBurningNotAllowed => write!(
                f,
                "native token burning is not allowed by the transaction capabilities."
            ),
            Self::CapabilitiesManaBurningNotAllowed => {
                write!(f, "mana burning is not allowed by the transaction capabilities.")
            }
            Self::CapabilitiesAccountDestructionNotAllowed => {
                write!(f, "account destruction is not allowed by the transaction capabilities.")
            }
            Self::CapabilitiesAnchorDestructionNotAllowed => {
                write!(f, "anchor destruction is not allowed by the transaction capabilities.")
            }
            Self::CapabilitiesFoundryDestructionNotAllowed => {
                write!(f, "foundry destruction is not allowed by the transaction capabilities.")
            }
            Self::CapabilitiesNftDestructionNotAllowed => {
                write!(f, "NFT destruction is not allowed by the transaction capabilities.")
            }
            Self::SemanticValidationFailed => write!(f, "semantic validation failed."),
        }
    }
}

impl TryFrom<u8> for TransactionFailureReason {
    type Error = Error;

    fn try_from(c: u8) -> Result<Self, Self::Error> {
        Self::from_repr(c).ok_or(Self::Error::InvalidTransactionFailureReason(c))
    }
}
