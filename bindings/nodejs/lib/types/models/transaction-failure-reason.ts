// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/**
 * Reason for transaction failure.
 */
export enum TransactionFailureReason {
    None = 0,
    ConflictRejected = 1,
    InputAlreadySpent = 2,
    InputCreationAfterTxCreation = 3,
    UnlockSignatureInvalid = 4,
    CommitmentInputReferenceInvalid = 5,
    BicInputReferenceInvalid = 6,
    RewardInputReferenceInvalid = 7,
    StakingRewardCalculationFailure = 8,
    DelegationRewardCalculationFailure = 9,
    InputOutputBaseTokenMismatch = 10,
    ManaOverflow = 11,
    InputOutputManaMismatch = 12,
    ManaDecayCreationIndexExceedsTargetIndex = 13,
    NativeTokenSumUnbalanced = 14,
    SimpleTokenSchemeMintedMeltedTokenDecrease = 15,
    SimpleTokenSchemeMintingInvalid = 16,
    SimpleTokenSchemeMeltingInvalid = 17,
    SimpleTokenSchemeMaximumSupplyChanged = 18,
    SimpleTokenSchemeGenesisInvalid = 19,
    MultiAddressLengthUnlockLengthMismatch = 20,
    MultiAddressUnlockThresholdNotReached = 21,
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
    AnchorInvalidStateTransition = 48,
    AnchorInvalidGovernanceTransition = 49,
    FoundryTransitionWithoutAccount = 50,
    FoundrySerialInvalid = 51,
    DelegationCommitmentInputMissing = 52,
    DelegationRewardInputMissing = 53,
    DelegationRewardsClaimingInvalid = 54,
    DelegationOutputTransitionedTwice = 55,
    DelegationModified = 56,
    DelegationStartEpochInvalid = 57,
    DelegationAmountMismatch = 58,
    DelegationEndEpochNotZero = 59,
    DelegationEndEpochInvalid = 60,
    CapabilitiesNativeTokenBurningNotAllowed = 61,
    CapabilitiesManaBurningNotAllowed = 62,
    CapabilitiesAccountDestructionNotAllowed = 63,
    CapabilitiesAnchorDestructionNotAllowed = 64,
    CapabilitiesFoundryDestructionNotAllowed = 65,
    CapabilitiesNftDestructionNotAllowed = 66,
    SemanticValidationFailed = 255,
}

/**
 * Transaction failure reason strings.
 */
export const TRANSACTION_FAILURE_REASON_STRINGS: {
    [key in TransactionFailureReason]: string;
} = {
    [TransactionFailureReason.None]: 'None.',
    [TransactionFailureReason.ConflictRejected]: 'Transaction is conflicting.',
    [TransactionFailureReason.InputAlreadySpent]: 'Input already spent.',
    [TransactionFailureReason.InputCreationAfterTxCreation]:
        'Input creation slot after tx creation slot.',
    [TransactionFailureReason.UnlockSignatureInvalid]:
        'Signature in unlock is invalid.',
    [TransactionFailureReason.CommitmentInputReferenceInvalid]:
        'Commitment input references an invalid or non-existent commitment.',
    [TransactionFailureReason.BicInputReferenceInvalid]:
        'BIC input reference cannot be loaded.',
    [TransactionFailureReason.RewardInputReferenceInvalid]:
        'Reward input does not reference a staking account or a delegation output.',
    [TransactionFailureReason.StakingRewardCalculationFailure]:
        'Staking rewards could not be calculated due to storage issues or overflow.',
    [TransactionFailureReason.DelegationRewardCalculationFailure]:
        'Delegation rewards could not be calculated due to storage issues or overflow.',
    [TransactionFailureReason.InputOutputBaseTokenMismatch]:
        'Inputs and outputs do not spend/deposit the same amount of base tokens.',
    [TransactionFailureReason.ManaOverflow]:
        'Under- or overflow in Mana calculations.',
    [TransactionFailureReason.InputOutputManaMismatch]:
        'Inputs and outputs do not contain the same amount of Mana.',
    [TransactionFailureReason.ManaDecayCreationIndexExceedsTargetIndex]:
        'Mana decay creation slot/epoch index exceeds target slot/epoch index.',
    [TransactionFailureReason.NativeTokenSumUnbalanced]:
        'Native token sums are unbalanced.',
    [TransactionFailureReason.SimpleTokenSchemeMintedMeltedTokenDecrease]:
        'Simple token scheme minted/melted value decreased.',
    [TransactionFailureReason.SimpleTokenSchemeMintingInvalid]:
        'Simple token scheme minting invalid.',
    [TransactionFailureReason.SimpleTokenSchemeMeltingInvalid]:
        'Simple token scheme melting invalid.',
    [TransactionFailureReason.SimpleTokenSchemeMaximumSupplyChanged]:
        'Simple token scheme maximum supply changed.',
    [TransactionFailureReason.SimpleTokenSchemeGenesisInvalid]:
        'Simple token scheme genesis invalid.',
    [TransactionFailureReason.MultiAddressLengthUnlockLengthMismatch]:
        'Multi address length and multi unlock length do not match.',
    [TransactionFailureReason.MultiAddressUnlockThresholdNotReached]:
        'Multi address unlock threshold not reached.',
    [TransactionFailureReason.SenderFeatureNotUnlocked]:
        'Sender feature is not unlocked.',
    [TransactionFailureReason.IssuerFeatureNotUnlocked]:
        'Issuer feature is not unlocked.',
    [TransactionFailureReason.StakingRewardInputMissing]:
        'Staking feature removal or resetting requires a reward input.',
    [TransactionFailureReason.StakingBlockIssuerFeatureMissing]:
        'Block issuer feature missing for account with staking feature.',
    [TransactionFailureReason.StakingCommitmentInputMissing]:
        'Staking feature validation requires a commitment input.',
    [TransactionFailureReason.StakingRewardClaimingInvalid]:
        'Staking feature must be removed or reset in order to claim rewards.',
    [TransactionFailureReason.StakingFeatureRemovedBeforeUnbonding]:
        'Staking feature can only be removed after the unbonding period.',
    [TransactionFailureReason.StakingFeatureModifiedBeforeUnbonding]:
        'Staking start epoch, fixed cost and staked amount cannot be modified while bonded.',
    [TransactionFailureReason.StakingStartEpochInvalid]:
        'Staking start epoch must be the epoch of the transaction.',
    [TransactionFailureReason.StakingEndEpochTooEarly]:
        'Staking end epoch must be set to the transaction epoch plus the unbonding period.',
    [TransactionFailureReason.BlockIssuerCommitmentInputMissing]:
        'Commitment input missing for block issuer feature.',
    [TransactionFailureReason.BlockIssuanceCreditInputMissing]:
        'Block issuance credit input missing for account with block issuer feature.',
    [TransactionFailureReason.BlockIssuerNotExpired]:
        'Block issuer feature has not expired.',
    [TransactionFailureReason.BlockIssuerExpiryTooEarly]:
        'Block issuer feature expiry set too early.',
    [TransactionFailureReason.ManaMovedOffBlockIssuerAccount]:
        'Mana cannot be moved off block issuer accounts except with manalocks.',
    [TransactionFailureReason.AccountLocked]:
        'Account is locked due to negative block issuance credits.',
    [TransactionFailureReason.TimelockCommitmentInputMissing]:
        "Transaction's containing a timelock condition require a commitment input.",
    [TransactionFailureReason.TimelockNotExpired]: 'Timelock not expired.',
    [TransactionFailureReason.ExpirationCommitmentInputMissing]:
        "Transaction's containing an expiration condition require a commitment input.",
    [TransactionFailureReason.ExpirationNotUnlockable]:
        'Expiration unlock condition cannot be unlocked.',
    [TransactionFailureReason.ReturnAmountNotFulFilled]:
        'Return amount not fulfilled.',
    [TransactionFailureReason.NewChainOutputHasNonZeroedId]:
        'New chain output has non-zeroed ID.',
    [TransactionFailureReason.ChainOutputImmutableFeaturesChanged]:
        'Immutable features in chain output modified during transition.',
    [TransactionFailureReason.ImplicitAccountDestructionDisallowed]:
        'Cannot destroy implicit account; must be transitioned to account.',
    [TransactionFailureReason.MultipleImplicitAccountCreationAddresses]:
        'Multiple implicit account creation addresses on the input side.',
    [TransactionFailureReason.AccountInvalidFoundryCounter]:
        'Foundry counter in account decreased or did not increase by the number of new foundries.',
    [TransactionFailureReason.AnchorInvalidStateTransition]:
        'Anchor has an invalid state transition.',
    [TransactionFailureReason.AnchorInvalidGovernanceTransition]:
        'Anchor has an invalid governance transition.',
    [TransactionFailureReason.FoundryTransitionWithoutAccount]:
        'Foundry output transitioned without accompanying account on input or output side.',
    [TransactionFailureReason.FoundrySerialInvalid]:
        'Foundry output serial number is invalid.',
    [TransactionFailureReason.DelegationCommitmentInputMissing]:
        'Delegation output validation requires a commitment input.',
    [TransactionFailureReason.DelegationRewardInputMissing]:
        'Delegation output cannot be destroyed without a reward input.',
    [TransactionFailureReason.DelegationRewardsClaimingInvalid]:
        'Invalid delegation mana rewards claiming.',
    [TransactionFailureReason.DelegationOutputTransitionedTwice]:
        'Delegation output attempted to be transitioned twice.',
    [TransactionFailureReason.DelegationModified]:
        'Delegated amount, validator ID and start epoch cannot be modified.',
    [TransactionFailureReason.DelegationStartEpochInvalid]:
        'Invalid start epoch.',
    [TransactionFailureReason.DelegationAmountMismatch]:
        'Delegated amount does not match amount.',
    [TransactionFailureReason.DelegationEndEpochNotZero]:
        'End epoch must be set to zero at output genesis.',
    [TransactionFailureReason.DelegationEndEpochInvalid]:
        'Delegation end epoch does not match current epoch.',
    [TransactionFailureReason.CapabilitiesNativeTokenBurningNotAllowed]:
        'Native token burning is not allowed by the transaction capabilities.',
    [TransactionFailureReason.CapabilitiesManaBurningNotAllowed]:
        'Mana burning is not allowed by the transaction capabilities.',
    [TransactionFailureReason.CapabilitiesAccountDestructionNotAllowed]:
        'Account destruction is not allowed by the transaction capabilities.',
    [TransactionFailureReason.CapabilitiesAnchorDestructionNotAllowed]:
        'Anchor destruction is not allowed by the transaction capabilities.',
    [TransactionFailureReason.CapabilitiesFoundryDestructionNotAllowed]:
        'Foundry destruction is not allowed by the transaction capabilities.',
    [TransactionFailureReason.CapabilitiesNftDestructionNotAllowed]:
        'NFT destruction is not allowed by the transaction capabilities.',
    [TransactionFailureReason.SemanticValidationFailed]:
        'Semantic validation failed.',
};
