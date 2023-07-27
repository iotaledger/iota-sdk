// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/**
 * Reason for block conflicts.
 */
export enum ConflictReason {
    /**
     * The block has no conflict.
     */
    none = 0,
    /**
     * The referenced UTXO was already spent.
     */
    inputUTXOAlreadySpent = 1,
    /**
     * The referenced UTXO was already spent while confirming this milestone.
     */
    inputUTXOAlreadySpentInThisMilestone = 2,
    /**
     * The referenced UTXO cannot be found.
     */
    inputUTXONotFound = 3,
    /**
     * The sum of the inputs and output values does not match.
     */
    inputOutputSumMismatch = 4,
    /**
     * The unlock signature is invalid.
     */
    invalidSignature = 5,
    /**
     * The configured timelock is not yet expired.
     */
    invalidTimelock = 6,
    /**
     * The native tokens are invalid.
     */
    invalidNativeTokens = 7,
    /**
     * The return amount in a transaction is not fulfilled by the output side.
     */
    returnAmountMismatch = 8,
    /**
     * The input unlock is invalid.
     */
    invalidInputUnlock = 9,
    /**
     * The inputs commitment is invalid.
     */
    invalidInputsCommitment = 10,
    /**
     * The output contains a Sender with an ident (address) which is not unlocked.
     */
    invalidSender = 11,
    /**
     * The chain state transition is invalid.
     */
    invalidChainState = 12,
    /**
     * The semantic validation failed.
     */
    semanticValidationFailed = 255,
}

/**
 * Conflict reason strings.
 */
export const CONFLICT_REASON_STRINGS: { [key in ConflictReason]: string } = {
    [ConflictReason.none]: 'The block has no conflict',
    [ConflictReason.inputUTXOAlreadySpent]:
        'The referenced UTXO was already spent',
    [ConflictReason.inputUTXOAlreadySpentInThisMilestone]:
        'The referenced UTXO was already spent while confirming this milestone',
    [ConflictReason.inputUTXONotFound]: 'The referenced UTXO cannot be found',
    [ConflictReason.inputOutputSumMismatch]:
        'The sum of the inputs and output values does not match',
    [ConflictReason.invalidSignature]: 'The unlock block signature is invalid',
    [ConflictReason.invalidTimelock]:
        'The configured timelock is not yet expired',
    [ConflictReason.invalidNativeTokens]: 'The native tokens are invalid',
    [ConflictReason.returnAmountMismatch]:
        'The return amount in a transaction is not fulfilled by the output side',
    [ConflictReason.invalidInputUnlock]: 'The input unlock is invalid',
    [ConflictReason.invalidInputsCommitment]:
        'The inputs commitment is invalid',
    [ConflictReason.invalidSender]:
        ' The output contains a Sender with an ident (address) which is not unlocked',
    [ConflictReason.invalidChainState]: 'The chain state transition is invalid',
    [ConflictReason.semanticValidationFailed]: 'The semantic validation failed',
};
