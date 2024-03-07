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
    StakingCommitmentInputMissing = 28,
    StakingRewardClaimingInvalid = 29,
    StakingFeatureRemovedBeforeUnbonding = 30,
    StakingFeatureModifiedBeforeUnbonding = 31,
    StakingStartEpochInvalid = 32,
    StakingEndEpochTooEarly = 33,
    BlockIssuerCommitmentInputMissing = 34,
    BlockIssuanceCreditInputMissing = 35,
    BlockIssuerNotExpired = 36,
    BlockIssuerExpiryTooEarly = 37,
    ManaMovedOffBlockIssuerAccount = 38,
    AccountLocked = 39,
    TimelockCommitmentInputMissing = 40,
    TimelockNotExpired = 41,
    ExpirationCommitmentInputMissing = 42,
    ExpirationNotUnlockable = 43,
    ReturnAmountNotFulFilled = 44,
    NewChainOutputHasNonZeroedId = 45,
    ChainOutputImmutableFeaturesChanged = 46,
    ImplicitAccountDestructionDisallowed = 47,
    MultipleImplicitAccountCreationAddresses = 48,
    AccountInvalidFoundryCounter = 49,
    AnchorInvalidStateTransition = 50,
    AnchorInvalidGovernanceTransition = 51,
    FoundryTransitionWithoutAccount = 52,
    FoundrySerialInvalid = 53,
    DelegationCommitmentInputMissing = 54,
    DelegationRewardInputMissing = 55,
    DelegationRewardsClaimingInvalid = 56,
    DelegationOutputTransitionedTwice = 57,
    DelegationModified = 58,
    DelegationStartEpochInvalid = 59,
    DelegationAmountMismatch = 60,
    DelegationEndEpochNotZero = 61,
    DelegationEndEpochInvalid = 62,
    CapabilitiesNativeTokenBurningNotAllowed = 63,
    CapabilitiesManaBurningNotAllowed = 64,
    CapabilitiesAccountDestructionNotAllowed = 65,
    CapabilitiesAnchorDestructionNotAllowed = 66,
    CapabilitiesFoundryDestructionNotAllowed = 67,
    CapabilitiesNftDestructionNotAllowed = 68,
    SemanticValidationFailed = 255,
}

/**
 * Transaction failure reason strings.
 */
export const TRANSACTION_FAILURE_REASON_STRINGS: {
    [key in TransactionFailureReason]: string;
} = {
    [TransactionFailureReason.None]: 'None.',
    [TransactionFailureReason.ConflictRejected]:
        'Transaction was conflicting and was rejected.',
    [TransactionFailureReason.InputAlreadySpent]: 'Input already spent.',
    [TransactionFailureReason.InputCreationAfterTxCreation]:
        'Input creation slot after tx creation slot.',
    [TransactionFailureReason.UnlockSignatureInvalid]:
        'Signature in unlock is invalid.',
    [TransactionFailureReason.ChainAddressUnlockInvalid]:
        'invalid unlock for chain address.',
    [TransactionFailureReason.DirectUnlockableAddressUnlockInvalid]:
        'invalid unlock for direct unlockable address.',
    [TransactionFailureReason.MultiAddressUnlockInvalid]:
        'invalid unlock for multi address.',
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
        "Simple token scheme's minted or melted tokens decreased.",
    [TransactionFailureReason.SimpleTokenSchemeMintingInvalid]:
        "Simple token scheme's minted tokens did not increase by the minted amount or melted tokens changed.",
    [TransactionFailureReason.SimpleTokenSchemeMeltingInvalid]:
        "Simple token scheme's melted tokens did not increase by the melted amount or minted tokens changed.",
    [TransactionFailureReason.SimpleTokenSchemeMaximumSupplyChanged]:
        "Simple token scheme's maximum supply cannot change during transition.",
    [TransactionFailureReason.SimpleTokenSchemeGenesisInvalid]:
        "Newly created simple token scheme's melted tokens are not zero or minted tokens do not equal native token amount in transaction.",
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
        'Invalid anchor state transition.',
    [TransactionFailureReason.AnchorInvalidGovernanceTransition]:
        'Invalid anchor governance transition.',
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
        'Delegation output has invalid start epoch.',
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
