// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/**
 * Reason for block failure.
 */
export enum BlockFailureReason {
    /** The block is too old to issue. */
    TooOldToIssue = 1,
    /** One of the block's parents is too old. */
    ParentTooOld = 2,
    /** One of the block's parents does not exist. */
    ParentDoesNotExist = 3,
    /** The block's issuer account could not be found. */
    IssuerAccountNotFound = 4,
    /** The mana cost could not be calculated. */
    ManaCostCalculationFailed = 5,
    /** The block's issuer account burned insufficient Mana for a block. */
    BurnedInsufficientMana = 6,
    /** The account is locked. */
    AccountLocked = 7,
    /** The account is expired. */
    AccountExpired = 8,
    /** The block's signature is invalid. */
    SignatureInvalid = 9,
    /** The block is dropped due to congestion. */
    DroppedDueToCongestion = 10,
    /** The block payload is invalid. */
    PayloadInvalid = 11,
    /** The block is invalid. */
    Invalid = 255,
}

/**
 * Block failure reason strings.
 */
export const BLOCK_FAILURE_REASON_STRINGS: {
    [key in BlockFailureReason]: string;
} = {
    [BlockFailureReason.TooOldToIssue]: 'The block is too old to issue.',
    [BlockFailureReason.ParentTooOld]: "One of the block's parents is too old.",
    [BlockFailureReason.ParentDoesNotExist]:
        "One of the block's parents does not exist.",
    [BlockFailureReason.IssuerAccountNotFound]:
        "The block's issuer account could not be found.",
    [BlockFailureReason.ManaCostCalculationFailed]:
        'The mana cost could not be calculated.',
    [BlockFailureReason.BurnedInsufficientMana]:
        "The block's issuer account burned insufficient Mana for a block.",
    [BlockFailureReason.AccountLocked]: 'The account is locked.',
    [BlockFailureReason.AccountExpired]: 'The account is expired.',
    [BlockFailureReason.SignatureInvalid]: "The block's signature is invalid.",
    [BlockFailureReason.DroppedDueToCongestion]:
        'The block is dropped due to congestion.',
    [BlockFailureReason.PayloadInvalid]: 'The block payload is invalid.',
    [BlockFailureReason.Invalid]: 'The block is invalid.',
};
