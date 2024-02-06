// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/**
 * Reason for transaction failure.
 */
export enum TransactionFailureReason {
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

/**
 * Transaction failure reason strings.
 */
export const TRANSACTION_FAILURE_REASON_STRINGS: {
    [key in TransactionFailureReason]: string;
} = {
    [TransactionFailureReason.None]: 'None.',
    [TransactionFailureReason.TypeInvalid]: 'Transaction type is invalid.',
    [TransactionFailureReason.Conflicting]: 'Transaction is conflicting.',
    [TransactionFailureReason.InputAlreadySpent]: 'Input already spent.',
    [TransactionFailureReason.InputCreationAfterTxCreation]:
        'Input creation slot after tx creation slot.',
    [TransactionFailureReason.UnlockSignatureInvalid]:
        'Signature in unlock is invalid.',
    [TransactionFailureReason.CommitmentInputMissing]:
        'Commitment input required with reward or BIC input.',
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
    [TransactionFailureReason.NativeTokenAmountLessThanZero]:
        'Native token amount must be greater than zero.',
    [TransactionFailureReason.NativeTokenSumExceedsUint256]:
        'Native token sum exceeds max value of a uint256.',
    [TransactionFailureReason.NativeTokenSumUnbalanced]:
        'Native token sums are unbalanced.',
    [TransactionFailureReason.MultiAddressLengthUnlockLengthMismatch]:
        'Multi address length and multi unlock length do not match.',
    [TransactionFailureReason.MultiAddressUnlockThresholdNotReached]:
        'Multi address unlock threshold not reached.',
    [TransactionFailureReason.NestedMultiUnlock]:
        "Multi unlocks can't be nested.",
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
