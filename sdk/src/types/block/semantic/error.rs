// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::convert::Infallible;

use crate::types::block::output::OutputError;

#[derive(Debug, PartialEq, Eq, strum::Display, derive_more::From)]
#[allow(missing_docs)]
pub enum SemanticError {
    #[strum(to_string = "{0}")]
    Output(OutputError),
    ConsumedAmountOverflow,
    CreatedAmountOverflow,
    CreatedManaOverflow,
    ConsumedManaOverflow,
    StorageDepositReturnOverflow,
    CreatedNativeTokensAmountOverflow,
    ConsumedNativeTokensAmountOverflow,
    InvalidTransactionFailureReason(u8),
    #[from]
    Reason(TransactionFailureReason),
}

impl SemanticError {
    pub fn transaction_failure_reason(&self) -> TransactionFailureReason {
        if let Self::Reason(reason) = self {
            *reason
        } else {
            TransactionFailureReason::SemanticValidationFailed
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for SemanticError {}

impl From<OutputError> for SemanticError {
    fn from(error: OutputError) -> Self {
        Self::Output(error)
    }
}

impl From<Infallible> for SemanticError {
    fn from(error: Infallible) -> Self {
        match error {}
    }
}

/// Describes the reason of a transaction failure.
#[repr(u8)]
#[derive(
    Debug,
    Copy,
    Clone,
    Eq,
    PartialEq,
    packable::Packable,
    strum::FromRepr,
    strum::EnumString,
    strum::Display,
    strum::AsRefStr,
)]
#[cfg_attr(feature = "serde", derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr))]
#[strum(serialize_all = "camelCase")]
#[packable(unpack_error = SemanticError)]
#[packable(tag_type = u8, with_error = SemanticError::InvalidTransactionFailureReason)]
#[non_exhaustive]
pub enum TransactionFailureReason {
    #[strum(to_string = "none")]
    None = 0,
    #[strum(to_string = "transaction was conflicting and was rejected")]
    ConflictRejected = 1,
    #[strum(to_string = "input already spent")]
    InputAlreadySpent = 2,
    #[strum(to_string = "input creation slot after tx creation slot")]
    InputCreationAfterTxCreation = 3,
    #[strum(to_string = "signature in unlock is invalid")]
    UnlockSignatureInvalid = 4,
    #[strum(to_string = "invalid unlock for chain address")]
    ChainAddressUnlockInvalid = 5,
    #[strum(to_string = "invalid unlock for direct unlockable address")]
    DirectUnlockableAddressUnlockInvalid = 6,
    #[strum(to_string = "invalid unlock for multi address")]
    MultiAddressUnlockInvalid = 7,
    #[strum(to_string = "commitment input references an invalid or non-existent commitment")]
    CommitmentInputReferenceInvalid = 8,
    #[strum(to_string = "BIC input reference cannot be loaded")]
    BicInputReferenceInvalid = 9,
    #[strum(to_string = "reward input does not reference a staking account or a delegation output")]
    RewardInputReferenceInvalid = 10,
    #[strum(to_string = "staking rewards could not be calculated due to storage issues or overflow")]
    StakingRewardCalculationFailure = 11,
    #[strum(to_string = "delegation rewards could not be calculated due to storage issues or overflow")]
    DelegationRewardCalculationFailure = 12,
    #[strum(to_string = "inputs and outputs do not spend/deposit the same amount of base tokens")]
    InputOutputBaseTokenMismatch = 13,
    #[strum(to_string = "under- or overflow in Mana calculations")]
    ManaOverflow = 14,
    #[strum(to_string = "inputs and outputs do not contain the same amount of mana")]
    InputOutputManaMismatch = 15,
    #[strum(to_string = "mana decay creation slot/epoch index exceeds target slot/epoch index")]
    ManaDecayCreationIndexExceedsTargetIndex = 16,
    #[strum(to_string = "native token sums are unbalanced")]
    NativeTokenSumUnbalanced = 17,
    #[strum(to_string = "simple token scheme's minted or melted tokens decreased")]
    SimpleTokenSchemeMintedMeltedTokenDecrease = 18,
    #[strum(
        to_string = "simple token scheme's minted tokens did not increase by the minted amount or melted tokens changed"
    )]
    SimpleTokenSchemeMintingInvalid = 19,
    #[strum(
        to_string = "simple token scheme's melted tokens did not increase by the melted amount or minted tokens changed"
    )]
    SimpleTokenSchemeMeltingInvalid = 20,
    #[strum(to_string = "simple token scheme's maximum supply cannot change during transition")]
    SimpleTokenSchemeMaximumSupplyChanged = 21,
    #[strum(
        to_string = "newly created simple token scheme's melted tokens are not zero or minted tokens do not equal native token amount in transaction"
    )]
    SimpleTokenSchemeGenesisInvalid = 22,
    #[strum(to_string = "multi address length and multi unlock length do not match")]
    MultiAddressLengthUnlockLengthMismatch = 23,
    #[strum(to_string = "multi address unlock threshold not reached")]
    MultiAddressUnlockThresholdNotReached = 24,
    #[strum(to_string = "sender feature is not unlocked")]
    SenderFeatureNotUnlocked = 25,
    #[strum(to_string = "issuer feature is not unlocked")]
    IssuerFeatureNotUnlocked = 26,
    #[strum(to_string = "staking feature removal or resetting requires a reward input")]
    StakingRewardInputMissing = 27,
    #[strum(to_string = "block issuer feature missing for account with staking feature")]
    StakingBlockIssuerFeatureMissing = 28,
    #[strum(to_string = "staking feature validation requires a commitment input")]
    StakingCommitmentInputMissing = 29,
    #[strum(to_string = "staking feature must be removed or reset in order to claim rewards")]
    StakingRewardClaimingInvalid = 30,
    #[strum(to_string = "staking feature can only be removed after the unbonding period")]
    StakingFeatureRemovedBeforeUnbonding = 31,
    #[strum(to_string = "staking start epoch, fixed cost and staked amount cannot be modified while bonded")]
    StakingFeatureModifiedBeforeUnbonding = 32,
    #[strum(to_string = "staking start epoch must be the epoch of the transaction")]
    StakingStartEpochInvalid = 33,
    #[strum(to_string = "staking end epoch must be set to the transaction epoch plus the unbonding period")]
    StakingEndEpochTooEarly = 34,
    #[strum(to_string = "commitment input missing for block issuer feature")]
    BlockIssuerCommitmentInputMissing = 35,
    #[strum(to_string = "block issuance credit input missing for account with block issuer feature")]
    BlockIssuanceCreditInputMissing = 36,
    #[strum(to_string = "block issuer feature has not expired")]
    BlockIssuerNotExpired = 37,
    #[strum(to_string = "block issuer feature expiry set too early")]
    BlockIssuerExpiryTooEarly = 38,
    #[strum(to_string = "mana cannot be moved off block issuer accounts except with manalocks")]
    ManaMovedOffBlockIssuerAccount = 39,
    #[strum(to_string = "account is locked due to negative block issuance credits")]
    AccountLocked = 40,
    #[strum(to_string = "transaction's containing a timelock condition require a commitment input")]
    TimelockCommitmentInputMissing = 41,
    #[strum(to_string = "timelock not expired")]
    TimelockNotExpired = 42,
    #[strum(to_string = "transaction's containing an expiration condition require a commitment input")]
    ExpirationCommitmentInputMissing = 43,
    #[strum(to_string = "expiration unlock condition cannot be unlocked")]
    ExpirationNotUnlockable = 44,
    #[strum(to_string = "return amount not fulfilled")]
    ReturnAmountNotFulFilled = 45,
    #[strum(to_string = "new chain output has non-zeroed ID")]
    NewChainOutputHasNonZeroedId = 46,
    #[strum(to_string = "immutable features in chain output modified during transition")]
    ChainOutputImmutableFeaturesChanged = 47,
    #[strum(to_string = "cannot destroy implicit account; must be transitioned to account")]
    ImplicitAccountDestructionDisallowed = 48,
    #[strum(to_string = "multiple implicit account creation addresses on the input side")]
    MultipleImplicitAccountCreationAddresses = 49,
    #[strum(to_string = "foundry counter in account decreased or did not increase by the number of new foundries")]
    AccountInvalidFoundryCounter = 50,
    #[strum(to_string = "invalid anchor state transition")]
    AnchorInvalidStateTransition = 51,
    #[strum(to_string = "invalid anchor governance transition")]
    AnchorInvalidGovernanceTransition = 52,
    #[strum(to_string = "foundry output transitioned without accompanying account on input or output side")]
    FoundryTransitionWithoutAccount = 53,
    #[strum(to_string = "foundry output serial number is invalid")]
    FoundrySerialInvalid = 54,
    #[strum(to_string = "delegation output validation requires a commitment input")]
    DelegationCommitmentInputMissing = 55,
    #[strum(to_string = "delegation output cannot be destroyed without a reward input")]
    DelegationRewardInputMissing = 56,
    #[strum(to_string = "invalid delegation mana rewards claiming")]
    DelegationRewardsClaimingInvalid = 57,
    #[strum(to_string = "attempted to transition delegation output twice")]
    DelegationOutputTransitionedTwice = 58,
    #[strum(to_string = "delegated amount, validator ID and start epoch cannot be modified")]
    DelegationModified = 59,
    #[strum(to_string = "delegation output has invalid start epoch")]
    DelegationStartEpochInvalid = 60,
    #[strum(to_string = "delegated amount does not match amount")]
    DelegationAmountMismatch = 61,
    #[strum(to_string = "end epoch must be set to zero at output genesis")]
    DelegationEndEpochNotZero = 62,
    #[strum(to_string = "delegation end epoch does not match current epoch")]
    DelegationEndEpochInvalid = 63,
    #[strum(to_string = "native token burning is not allowed by the transaction capabilities")]
    CapabilitiesNativeTokenBurningNotAllowed = 64,
    #[strum(to_string = "mana burning is not allowed by the transaction capabilities")]
    CapabilitiesManaBurningNotAllowed = 65,
    #[strum(to_string = "account destruction is not allowed by the transaction capabilities")]
    CapabilitiesAccountDestructionNotAllowed = 66,
    #[strum(to_string = "anchor destruction is not allowed by the transaction capabilities")]
    CapabilitiesAnchorDestructionNotAllowed = 67,
    #[strum(to_string = "foundry destruction is not allowed by the transaction capabilities")]
    CapabilitiesFoundryDestructionNotAllowed = 68,
    #[strum(to_string = "NFT destruction is not allowed by the transaction capabilities")]
    CapabilitiesNftDestructionNotAllowed = 69,
    #[strum(to_string = "semantic validation failed")]
    SemanticValidationFailed = 255,
}

impl TryFrom<u8> for TransactionFailureReason {
    type Error = SemanticError;

    fn try_from(c: u8) -> Result<Self, Self::Error> {
        Self::from_repr(c).ok_or(Self::Error::InvalidTransactionFailureReason(c))
    }
}
