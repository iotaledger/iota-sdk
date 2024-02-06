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
    [TransactionFailureReason.None]: 'none.',
    [TransactionFailureReason.TypeInvalid]: 'transaction type is invalid.',
    [TransactionFailureReason.Conflicting]: 'transaction is conflicting.',
    [TransactionFailureReason.InputAlreadySpent]: 'input already spent.',
    [TransactionFailureReason.InputCreationAfterTxCreation]:
        'input creation slot after tx creation slot.',
    [TransactionFailureReason.UnlockSignatureInvalid]:
        'signature in unlock is invalid.',
    [TransactionFailureReason.CommitmentInputMissing]:
        'commitment input required with reward or BIC input.',
    [TransactionFailureReason.CommitmentInputReferenceInvalid]:
        'commitment input references an invalid or non-existent commitment.',
    [TransactionFailureReason.BicInputReferenceInvalid]:
        'BIC input reference cannot be loaded.',
    [TransactionFailureReason.RewardInputReferenceInvalid]:
        'reward input does not reference a staking account or a delegation output.',
    [TransactionFailureReason.StakingRewardCalculationFailure]:
        'staking rewards could not be calculated due to storage issues or overflow.',
    [TransactionFailureReason.DelegationRewardCalculationFailure]:
        'delegation rewards could not be calculated due to storage issues or overflow.',
    [TransactionFailureReason.InputOutputBaseTokenMismatch]:
        'inputs and outputs do not spend/deposit the same amount of base tokens.',
    [TransactionFailureReason.ManaOverflow]:
        'under- or overflow in Mana calculations.',
    [TransactionFailureReason.InputOutputManaMismatch]:
        'inputs and outputs do not contain the same amount of Mana.',
    [TransactionFailureReason.ManaDecayCreationIndexExceedsTargetIndex]:
        'mana decay creation slot/epoch index exceeds target slot/epoch index.',
    [TransactionFailureReason.NativeTokenAmountLessThanZero]:
        'native token amount must be greater than zero.',
    [TransactionFailureReason.NativeTokenSumExceedsUint256]:
        'native token sum exceeds max value of a uint256.',
    [TransactionFailureReason.NativeTokenSumUnbalanced]:
        'native token sums are unbalanced.',
    [TransactionFailureReason.MultiAddressLengthUnlockLengthMismatch]:
        'multi address length and multi unlock length do not match.',
    [TransactionFailureReason.MultiAddressUnlockThresholdNotReached]:
        'multi address unlock threshold not reached.',
    [TransactionFailureReason.NestedMultiUnlock]:
        "multi unlocks can't be nested.",
    [TransactionFailureReason.SenderFeatureNotUnlocked]:
        'sender feature is not unlocked.',
    [TransactionFailureReason.IssuerFeatureNotUnlocked]:
        'issuer feature is not unlocked.',
    [TransactionFailureReason.StakingRewardInputMissing]:
        'staking feature removal or resetting requires a reward input.',
    [TransactionFailureReason.StakingBlockIssuerFeatureMissing]:
        'block issuer feature missing for account with staking feature.',
    [TransactionFailureReason.StakingCommitmentInputMissing]:
        'staking feature validation requires a commitment input.',
    [TransactionFailureReason.StakingRewardClaimingInvalid]:
        'staking feature must be removed or reset in order to claim rewards.',
    [TransactionFailureReason.StakingFeatureRemovedBeforeUnbonding]:
        'staking feature can only be removed after the unbonding period.',
    [TransactionFailureReason.StakingFeatureModifiedBeforeUnbonding]:
        'staking start epoch, fixed cost and staked amount cannot be modified while bonded.',
    [TransactionFailureReason.StakingStartEpochInvalid]:
        'staking start epoch must be the epoch of the transaction.',
    [TransactionFailureReason.StakingEndEpochTooEarly]:
        'staking end epoch must be set to the transaction epoch plus the unbonding period.',
    [TransactionFailureReason.BlockIssuerCommitmentInputMissing]:
        'commitment input missing for block issuer feature.',
    [TransactionFailureReason.BlockIssuanceCreditInputMissing]:
        'block issuance credit input missing for account with block issuer feature.',
    [TransactionFailureReason.BlockIssuerNotExpired]:
        'block issuer feature has not expired.',
    [TransactionFailureReason.BlockIssuerExpiryTooEarly]:
        'block issuer feature expiry set too early.',
    [TransactionFailureReason.ManaMovedOffBlockIssuerAccount]:
        'mana cannot be moved off block issuer accounts except with manalocks.',
    [TransactionFailureReason.AccountLocked]:
        'account is locked due to negative block issuance credits.',
    [TransactionFailureReason.TimelockCommitmentInputMissing]:
        "transaction's containing a timelock condition require a commitment input.",
    [TransactionFailureReason.TimelockNotExpired]: 'timelock not expired.',
    [TransactionFailureReason.ExpirationCommitmentInputMissing]:
        "transaction's containing an expiration condition require a commitment input.",
    [TransactionFailureReason.ExpirationNotUnlockable]:
        'expiration unlock condition cannot be unlocked.',
    [TransactionFailureReason.ReturnAmountNotFulFilled]:
        'return amount not fulfilled.',
    [TransactionFailureReason.NewChainOutputHasNonZeroedId]:
        'new chain output has non-zeroed ID.',
    [TransactionFailureReason.ChainOutputImmutableFeaturesChanged]:
        'immutable features in chain output modified during transition.',
    [TransactionFailureReason.ImplicitAccountDestructionDisallowed]:
        'cannot destroy implicit account; must be transitioned to account.',
    [TransactionFailureReason.MultipleImplicitAccountCreationAddresses]:
        'multiple implicit account creation addresses on the input side.',
    [TransactionFailureReason.AccountInvalidFoundryCounter]:
        'foundry counter in account decreased or did not increase by the number of new foundries.',
    [TransactionFailureReason.FoundryTransitionWithoutAccount]:
        'foundry output transitioned without accompanying account on input or output side.',
    [TransactionFailureReason.FoundrySerialInvalid]:
        'foundry output serial number is invalid.',
    [TransactionFailureReason.DelegationCommitmentInputMissing]:
        'delegation output validation requires a commitment input.',
    [TransactionFailureReason.DelegationRewardInputMissing]:
        'delegation output cannot be destroyed without a reward input.',
    [TransactionFailureReason.DelegationRewardsClaimingInvalid]:
        'invalid delegation mana rewards claiming.',
    [TransactionFailureReason.DelegationOutputTransitionedTwice]:
        'delegation output attempted to be transitioned twice.',
    [TransactionFailureReason.DelegationModified]:
        'delegated amount, validator ID and start epoch cannot be modified.',
    [TransactionFailureReason.DelegationStartEpochInvalid]:
        'invalid start epoch.',
    [TransactionFailureReason.DelegationAmountMismatch]:
        'delegated amount does not match amount.',
    [TransactionFailureReason.DelegationEndEpochNotZero]:
        'end epoch must be set to zero at output genesis.',
    [TransactionFailureReason.DelegationEndEpochInvalid]:
        'delegation end epoch does not match current epoch.',
    [TransactionFailureReason.CapabilitiesNativeTokenBurningNotAllowed]:
        'native token burning is not allowed by the transaction capabilities.',
    [TransactionFailureReason.CapabilitiesManaBurningNotAllowed]:
        'mana burning is not allowed by the transaction capabilities.',
    [TransactionFailureReason.CapabilitiesAccountDestructionNotAllowed]:
        'account destruction is not allowed by the transaction capabilities.',
    [TransactionFailureReason.CapabilitiesAnchorDestructionNotAllowed]:
        'anchor destruction is not allowed by the transaction capabilities.',
    [TransactionFailureReason.CapabilitiesFoundryDestructionNotAllowed]:
        'foundry destruction is not allowed by the transaction capabilities.',
    [TransactionFailureReason.CapabilitiesNftDestructionNotAllowed]:
        'NFT destruction is not allowed by the transaction capabilities.',
    [TransactionFailureReason.SemanticValidationFailed]:
        'semantic validation failed.',
};
