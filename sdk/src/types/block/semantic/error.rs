// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::fmt;

use crate::types::block::Error;

/// Describes the reason of a transaction failure.
#[repr(u8)]
#[derive(
    Debug, Copy, Clone, Eq, PartialEq, packable::Packable, strum::FromRepr, strum::EnumString, strum::AsRefStr,
)]
#[cfg_attr(feature = "serde", derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr))]
#[strum(serialize_all = "camelCase")]
#[packable(unpack_error = Error)]
#[packable(tag_type = u8, with_error = Error::InvalidTransactionFailureReason)]
#[non_exhaustive]
pub enum TransactionFailureReason {
    None = 0,
    ConflictRejected = 1,
    InputAlreadySpent = 2,
    InputCreationAfterTxCreation = 3,
    UnlockSignatureInvalid = 4,
    ChainAddressUnlockInvalid = 5,
    DirectUnlockableAddressUnlockInvalid = 6,
    MultiAddressUnlockInvalid = 7,
    CommitmentInputReferenceInvalid = 8,
    BicInputReferenceInvalid = 9,
    RewardInputReferenceInvalid = 10,
    StakingRewardCalculationFailure = 11,
    DelegationRewardCalculationFailure = 12,
    InputOutputBaseTokenMismatch = 13,
    ManaOverflow = 14,
    InputOutputManaMismatch = 15,
    ManaDecayCreationIndexExceedsTargetIndex = 16,
    NativeTokenSumUnbalanced = 17,
    SimpleTokenSchemeMintedMeltedTokenDecrease = 18,
    SimpleTokenSchemeMintingInvalid = 19,
    SimpleTokenSchemeMeltingInvalid = 20,
    SimpleTokenSchemeMaximumSupplyChanged = 21,
    SimpleTokenSchemeGenesisInvalid = 22,
    MultiAddressLengthUnlockLengthMismatch = 23,
    MultiAddressUnlockThresholdNotReached = 24,
    SenderFeatureNotUnlocked = 25,
    IssuerFeatureNotUnlocked = 26,
    StakingRewardInputMissing = 27,
    StakingBlockIssuerFeatureMissing = 28,
    StakingCommitmentInputMissing = 29,
    StakingRewardClaimingInvalid = 30,
    StakingFeatureRemovedBeforeUnbonding = 31,
    StakingFeatureModifiedBeforeUnbonding = 32,
    StakingStartEpochInvalid = 33,
    StakingEndEpochTooEarly = 34,
    BlockIssuerCommitmentInputMissing = 35,
    BlockIssuanceCreditInputMissing = 36,
    BlockIssuerNotExpired = 37,
    BlockIssuerExpiryTooEarly = 38,
    ManaMovedOffBlockIssuerAccount = 39,
    AccountLocked = 40,
    TimelockCommitmentInputMissing = 41,
    TimelockNotExpired = 42,
    ExpirationCommitmentInputMissing = 43,
    ExpirationNotUnlockable = 44,
    ReturnAmountNotFulFilled = 45,
    NewChainOutputHasNonZeroedId = 46,
    ChainOutputImmutableFeaturesChanged = 47,
    ImplicitAccountDestructionDisallowed = 48,
    MultipleImplicitAccountCreationAddresses = 49,
    AccountInvalidFoundryCounter = 50,
    AnchorInvalidStateTransition = 51,
    AnchorInvalidGovernanceTransition = 52,
    FoundryTransitionWithoutAccount = 53,
    FoundrySerialInvalid = 54,
    DelegationCommitmentInputMissing = 55,
    DelegationRewardInputMissing = 56,
    DelegationRewardsClaimingInvalid = 57,
    DelegationOutputTransitionedTwice = 58,
    DelegationModified = 59,
    DelegationStartEpochInvalid = 60,
    DelegationAmountMismatch = 61,
    DelegationEndEpochNotZero = 62,
    DelegationEndEpochInvalid = 63,
    CapabilitiesNativeTokenBurningNotAllowed = 64,
    CapabilitiesManaBurningNotAllowed = 65,
    CapabilitiesAccountDestructionNotAllowed = 66,
    CapabilitiesAnchorDestructionNotAllowed = 67,
    CapabilitiesFoundryDestructionNotAllowed = 68,
    CapabilitiesNftDestructionNotAllowed = 69,
    SemanticValidationFailed = 255,
}

impl fmt::Display for TransactionFailureReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "none."),
            Self::ConflictRejected => write!(f, "transaction was conflicting and was rejected."),
            Self::InputAlreadySpent => write!(f, "input already spent."),
            Self::InputCreationAfterTxCreation => write!(f, "input creation slot after tx creation slot."),
            Self::UnlockSignatureInvalid => write!(f, "signature in unlock is invalid."),
            Self::ChainAddressUnlockInvalid => write!(f, "invalid unlock for chain address."),
            Self::DirectUnlockableAddressUnlockInvalid => write!(f, "invalid unlock for direct unlockable address."),
            Self::MultiAddressUnlockInvalid => write!(f, "invalid unlock for multi address."),
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
            Self::NativeTokenSumUnbalanced => write!(f, "native token sums are unbalanced."),
            Self::SimpleTokenSchemeMintedMeltedTokenDecrease => {
                write!(f, "simple token scheme's minted or melted tokens decreased.")
            }
            Self::SimpleTokenSchemeMintingInvalid => write!(
                f,
                "simple token scheme's minted tokens did not increase by the minted amount or melted tokens changed."
            ),
            Self::SimpleTokenSchemeMeltingInvalid => write!(
                f,
                "simple token scheme's melted tokens did not increase by the melted amount or minted tokens changed."
            ),
            Self::SimpleTokenSchemeMaximumSupplyChanged => write!(
                f,
                "simple token scheme's maximum supply cannot change during transition."
            ),
            Self::SimpleTokenSchemeGenesisInvalid => write!(
                f,
                "newly created simple token scheme's melted tokens are not zero or minted tokens do not equal native token amount in transaction."
            ),
            Self::MultiAddressLengthUnlockLengthMismatch => {
                write!(f, "multi address length and multi unlock length do not match.")
            }
            Self::MultiAddressUnlockThresholdNotReached => write!(f, "multi address unlock threshold not reached."),
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
            Self::AnchorInvalidStateTransition => write!(f, "invalid anchor state transition."),
            Self::AnchorInvalidGovernanceTransition => write!(f, "invalid anchor governance transition."),
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
            Self::DelegationStartEpochInvalid => write!(f, "delegation output has invalid start epoch."),
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
