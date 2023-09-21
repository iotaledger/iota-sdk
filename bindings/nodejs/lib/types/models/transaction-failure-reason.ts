// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/**
 * Reason for transaction failure..
 */
export enum TransactionFailureReason {
    /**
     * The referenced UTXO was already spent.
     */
    inputUTXOAlreadySpent = 1,

    /**
     * The transaction is conflicting with another transaction.
     * Conflicting specifically means a double spend situation that both transactions pass all validation rules,
     * eventually losing one(s) should have this reason.
     */
    conflictingWithAnotherTx = 2,

    /**
     * The referenced UTXO is invalid.
     */
    invalidReferencedUtxo = 3,

    /**
     * The transaction is invalid.
     */
    invalidTransaction = 4,

    /**
     * The sum of the inputs and output base token amount does not match.
     */
    sumInputsOutputsAmountMismatch = 5,

    /**
     * The unlock block signature is invalid.
     */
    invalidUnlockBlockSignature = 6,

    /**
     * The configured timelock is not yet expired.
     */
    timelockNotExpired = 7,

    /**
     * The given native tokens are invalid.
     */
    invalidNativeTokens = 8,

    /**
     * The return amount in a transaction is not fulfilled by the output side.
     */
    storageDepositReturnUnfulfilled = 9,

    /**
     * An input unlock was invalid.
     */
    invalidInputUnlock = 10,

    /**
     * The inputs commitment is invalid.
     */
    invalidInputsCommitment = 11,

    /**
     * The output contains a Sender with an ident (address) which is not unlocked.
     */
    senderNotUnlocked = 12,

    /**
     * The chain state transition is invalid.
     */
    invalidChainStateTransition = 13,

    /**
     * The referenced input is created after transaction issuing time.
     */
    invalidTransactionIssuingTime = 14,

    /**
     * The mana amount is invalid.
     */
    invalidManaAmount = 15,

    /**
     * The Block Issuance Credits amount is invalid.
     */
    invalidBlockIssuanceCreditsAmount = 16,

    /**
     * Reward Context Input is invalid.
     */
    invalidRewardContextInput = 17,

    /**
     * Commitment Context Input is invalid.
     */
    invalidCommitmentContextInput = 18,

    /**
     * Staking Feature is not provided in account output when claiming rewards.
     */
    missingStakingFeature = 19,

    /**
     * Failed to claim staking reward.
     */
    failedToClaimStakingReward = 20,

    /**
     * Failed to claim delegation reward.
     */
    failedToClaimDelegationReward = 21,

    /**
     * The semantic validation failed for a reason not covered by the previous variants.
     */
    semanticValidationFailed = 255,
}

/**
 * Transaction failure reason strings.
 */
export const TRANSACTION_FAILURE_REASON_STRINGS: {
    [key in TransactionFailureReason]: string;
} = {
    [TransactionFailureReason.inputUTXOAlreadySpent]:
        'The referenced UTXO was already spent.',
    [TransactionFailureReason.conflictingWithAnotherTx]:
        'The transaction is conflicting with another transaction. Conflicting specifically means a double spend situation that both transactions pass all validation rules, eventually losing one(s) should have this reason.',
    [TransactionFailureReason.invalidReferencedUtxo]:
        'The referenced UTXO is invalid.',
    [TransactionFailureReason.invalidTransaction]:
        'The transaction is invalid.',
    [TransactionFailureReason.sumInputsOutputsAmountMismatch]:
        'The sum of the inputs and output base token amount does not match.',
    [TransactionFailureReason.invalidUnlockBlockSignature]:
        'The unlock block signature is invalid.',
    [TransactionFailureReason.timelockNotExpired]:
        'The configured timelock is not yet expired.',
    [TransactionFailureReason.invalidNativeTokens]:
        'The given native tokens are invalid.',
    [TransactionFailureReason.storageDepositReturnUnfulfilled]:
        'The return amount in a transaction is not fulfilled by the output side.',
    [TransactionFailureReason.invalidInputUnlock]:
        'An input unlock was invalid.',
    [TransactionFailureReason.invalidInputsCommitment]:
        'The inputs commitment is invalid.',
    [TransactionFailureReason.senderNotUnlocked]:
        'The output contains a Sender with an ident (address) which is not unlocked.',
    [TransactionFailureReason.invalidChainStateTransition]:
        'The chain state transition is invalid.',
    [TransactionFailureReason.invalidTransactionIssuingTime]:
        'The referenced input is created after transaction issuing time.',
    [TransactionFailureReason.invalidManaAmount]: 'The mana amount is invalid.',
    [TransactionFailureReason.invalidBlockIssuanceCreditsAmount]:
        'The Block Issuance Credits amount is invalid.',
    [TransactionFailureReason.invalidRewardContextInput]:
        'Reward Context Input is invalid.',
    [TransactionFailureReason.invalidCommitmentContextInput]:
        'Commitment Context Input is invalid.',
    [TransactionFailureReason.missingStakingFeature]:
        'Staking Feature is not provided in account output when claiming rewards.',
    [TransactionFailureReason.failedToClaimStakingReward]:
        'Failed to claim staking reward.',
    [TransactionFailureReason.failedToClaimDelegationReward]:
        'Failed to claim delegation reward.',
    [TransactionFailureReason.semanticValidationFailed]:
        'The semantic validation failed for a reason not covered by the previous variants.',
};
