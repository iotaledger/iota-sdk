// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use core::convert::Infallible;

#[derive(Debug, PartialEq, Eq, derive_more::Display)]
#[display(fmt = "invalid transaction failure reason: {_0}")]
pub struct InvalidTransactionFailureReasonError(u8);

impl From<Infallible> for InvalidTransactionFailureReasonError {
    fn from(value: Infallible) -> Self {
        match value {}
    }
}

#[cfg(feature = "std")]
impl std::error::Error for TransactionFailureReason {}

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
    derive_more::Display,
    strum::AsRefStr,
)]
#[cfg_attr(feature = "serde", derive(serde_repr::Serialize_repr, serde_repr::Deserialize_repr))]
#[strum(serialize_all = "camelCase")]
#[packable(unpack_error = InvalidTransactionFailureReasonError)]
#[packable(tag_type = u8, with_error = InvalidTransactionFailureReasonError)]
#[non_exhaustive]
pub enum TransactionFailureReason {
    #[display(fmt = "none")]
    None = 0,
    #[display(fmt = "transaction was conflicting and was rejected")]
    ConflictRejected = 1,
    #[display(fmt = "transaction was orphaned")]
    Orphaned = 2,
    #[display(fmt = "input already spent")]
    InputAlreadySpent = 3,
    #[display(fmt = "input creation slot after tx creation slot")]
    InputCreationAfterTxCreation = 4,
    #[display(fmt = "signature in unlock is invalid")]
    UnlockSignatureInvalid = 5,
    #[display(fmt = "invalid unlock for chain address")]
    ChainAddressUnlockInvalid = 6,
    #[display(fmt = "invalid unlock for direct unlockable address")]
    DirectUnlockableAddressUnlockInvalid = 7,
    #[display(fmt = "invalid unlock for multi address")]
    MultiAddressUnlockInvalid = 8,
    #[display(fmt = "commitment input references an invalid or non-existent commitment")]
    CommitmentInputReferenceInvalid = 9,
    #[display(fmt = "BIC input reference cannot be loaded")]
    BicInputReferenceInvalid = 10,
    #[display(fmt = "reward input does not reference a staking account or a delegation output")]
    RewardInputReferenceInvalid = 11,
    #[display(fmt = "staking rewards could not be calculated due to storage issues or overflow")]
    StakingRewardCalculationFailure = 12,
    #[display(fmt = "delegation rewards could not be calculated due to storage issues or overflow")]
    DelegationRewardCalculationFailure = 13,
    #[display(fmt = "inputs and outputs do not spend/deposit the same amount of base tokens")]
    InputOutputBaseTokenMismatch = 14,
    #[display(fmt = "under- or overflow in Mana calculations")]
    ManaOverflow = 15,
    #[display(fmt = "inputs and outputs do not contain the same amount of mana")]
    InputOutputManaMismatch = 16,
    #[display(fmt = "mana decay creation slot/epoch index exceeds target slot/epoch index")]
    ManaDecayCreationIndexExceedsTargetIndex = 17,
    #[display(fmt = "native token sums are unbalanced")]
    NativeTokenSumUnbalanced = 18,
    #[display(fmt = "simple token scheme's minted or melted tokens decreased")]
    SimpleTokenSchemeMintedMeltedTokenDecrease = 19,
    #[strum(
        to_string = "simple token scheme's minted tokens did not increase by the minted amount or melted tokens changed"
    )]
    SimpleTokenSchemeMintingInvalid = 20,
    #[strum(
        to_string = "simple token scheme's melted tokens did not increase by the melted amount or minted tokens changed"
    )]
    SimpleTokenSchemeMeltingInvalid = 21,
    #[display(fmt = "simple token scheme's maximum supply cannot change during transition")]
    SimpleTokenSchemeMaximumSupplyChanged = 22,
    #[strum(
        to_string = "newly created simple token scheme's melted tokens are not zero or minted tokens do not equal native token amount in transaction"
    )]
    SimpleTokenSchemeGenesisInvalid = 23,
    #[display(fmt = "multi address length and multi unlock length do not match")]
    MultiAddressLengthUnlockLengthMismatch = 24,
    #[display(fmt = "multi address unlock threshold not reached")]
    MultiAddressUnlockThresholdNotReached = 25,
    #[display(fmt = "sender feature is not unlocked")]
    SenderFeatureNotUnlocked = 26,
    #[display(fmt = "issuer feature is not unlocked")]
    IssuerFeatureNotUnlocked = 27,
    #[display(fmt = "staking feature removal or resetting requires a reward input")]
    StakingRewardInputMissing = 28,
    #[display(fmt = "staking feature validation requires a commitment input")]
    StakingCommitmentInputMissing = 29,
    #[display(fmt = "staking feature must be removed or reset in order to claim rewards")]
    StakingRewardClaimingInvalid = 30,
    #[display(fmt = "staking feature can only be removed after the unbonding period")]
    StakingFeatureRemovedBeforeUnbonding = 31,
    #[display(fmt = "staking start epoch, fixed cost and staked amount cannot be modified while bonded")]
    StakingFeatureModifiedBeforeUnbonding = 32,
    #[display(fmt = "staking start epoch must be the epoch of the transaction")]
    StakingStartEpochInvalid = 33,
    #[display(fmt = "staking end epoch must be set to the transaction epoch plus the unbonding period")]
    StakingEndEpochTooEarly = 34,
    #[display(fmt = "commitment input missing for block issuer feature")]
    BlockIssuerCommitmentInputMissing = 35,
    #[display(fmt = "block issuance credit input missing for account with block issuer feature")]
    BlockIssuanceCreditInputMissing = 36,
    #[display(fmt = "block issuer feature has not expired")]
    BlockIssuerNotExpired = 37,
    #[display(fmt = "block issuer feature expiry set too early")]
    BlockIssuerExpiryTooEarly = 38,
    #[display(fmt = "mana cannot be moved off block issuer accounts except with manalocks")]
    ManaMovedOffBlockIssuerAccount = 39,
    #[display(fmt = "account is locked due to negative block issuance credits")]
    AccountLocked = 40,
    #[display(fmt = "transaction's containing a timelock condition require a commitment input")]
    TimelockCommitmentInputMissing = 41,
    #[display(fmt = "timelock not expired")]
    TimelockNotExpired = 42,
    #[display(fmt = "transaction's containing an expiration condition require a commitment input")]
    ExpirationCommitmentInputMissing = 43,
    #[display(fmt = "expiration unlock condition cannot be unlocked")]
    ExpirationNotUnlockable = 44,
    #[display(fmt = "return amount not fulfilled")]
    ReturnAmountNotFulFilled = 45,
    #[display(fmt = "new chain output has non-zeroed ID")]
    NewChainOutputHasNonZeroedId = 46,
    #[display(fmt = "immutable features in chain output modified during transition")]
    ChainOutputImmutableFeaturesChanged = 47,
    #[display(fmt = "cannot destroy implicit account; must be transitioned to account")]
    ImplicitAccountDestructionDisallowed = 48,
    #[display(fmt = "multiple implicit account creation addresses on the input side")]
    MultipleImplicitAccountCreationAddresses = 49,
    #[display(fmt = "foundry counter in account decreased or did not increase by the number of new foundries")]
    AccountInvalidFoundryCounter = 50,
    #[display(fmt = "invalid anchor state transition")]
    AnchorInvalidStateTransition = 51,
    #[display(fmt = "invalid anchor governance transition")]
    AnchorInvalidGovernanceTransition = 52,
    #[display(fmt = "foundry output transitioned without accompanying account on input or output side")]
    FoundryTransitionWithoutAccount = 53,
    #[display(fmt = "foundry output serial number is invalid")]
    FoundrySerialInvalid = 54,
    #[display(fmt = "delegation output validation requires a commitment input")]
    DelegationCommitmentInputMissing = 55,
    #[display(fmt = "delegation output cannot be destroyed without a reward input")]
    DelegationRewardInputMissing = 56,
    #[display(fmt = "invalid delegation mana rewards claiming")]
    DelegationRewardsClaimingInvalid = 57,
    #[display(fmt = "attempted to transition delegation output twice")]
    DelegationOutputTransitionedTwice = 58,
    #[display(fmt = "delegated amount, validator ID and start epoch cannot be modified")]
    DelegationModified = 59,
    #[display(fmt = "delegation output has invalid start epoch")]
    DelegationStartEpochInvalid = 60,
    #[display(fmt = "delegated amount does not match amount")]
    DelegationAmountMismatch = 61,
    #[display(fmt = "end epoch must be set to zero at output genesis")]
    DelegationEndEpochNotZero = 62,
    #[display(fmt = "delegation end epoch does not match current epoch")]
    DelegationEndEpochInvalid = 63,
    #[display(fmt = "native token burning is not allowed by the transaction capabilities")]
    CapabilitiesNativeTokenBurningNotAllowed = 64,
    #[display(fmt = "mana burning is not allowed by the transaction capabilities")]
    CapabilitiesManaBurningNotAllowed = 65,
    #[display(fmt = "account destruction is not allowed by the transaction capabilities")]
    CapabilitiesAccountDestructionNotAllowed = 66,
    #[display(fmt = "anchor destruction is not allowed by the transaction capabilities")]
    CapabilitiesAnchorDestructionNotAllowed = 67,
    #[display(fmt = "foundry destruction is not allowed by the transaction capabilities")]
    CapabilitiesFoundryDestructionNotAllowed = 68,
    #[display(fmt = "NFT destruction is not allowed by the transaction capabilities")]
    CapabilitiesNftDestructionNotAllowed = 69,
    #[display(fmt = "semantic validation failed")]
    SemanticValidationFailed = 255,
}

impl TryFrom<u8> for TransactionFailureReason {
    type Error = InvalidTransactionFailureReasonError;

    fn try_from(c: u8) -> Result<Self, Self::Error> {
        Self::from_repr(c).ok_or(InvalidTransactionFailureReasonError(c))
    }
}
