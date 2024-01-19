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
    /** One of the block's parents is invalid. */
    ParentInvalid = 4,
    /** The block's issuer account could not be found. */
    IssuerAccountNotFound = 5,
    /** The block's protocol version is invalid. */
    VersionInvalid = 6,
    /** The mana cost could not be calculated. */
    ManaCostCalculationFailed = 7,
    /** The block's issuer account burned insufficient Mana for a block. */
    BurnedInsufficientMana = 8,
    /** The account is invalid. */
    AccountInvalid = 9,
    /** The block's signature is invalid. */
    SignatureInvalid = 10,
    /** The block is dropped due to congestion. */
    DroppedDueToCongestion = 11,
    /** The block payload is invalid. */
    PayloadInvalid = 12,
    /** The block is invalid. */
    Invalid = 255,
}

/**
 * Transaction failure reason strings.
 */
export const BLOCK_FAILURE_REASON_STRINGS: {
    [key in BlockFailureReason]: string;
} = {
    [BlockFailureReason.TooOldToIssue]: 'The block is too old to issue.',
    [BlockFailureReason.ParentTooOld]: "One of the block's parents is too old.",
    [BlockFailureReason.ParentDoesNotExist]:
        "One of the block's parents does not exist.",
    [BlockFailureReason.ParentInvalid]:
        "One of the block's parents is invalid.",
    [BlockFailureReason.IssuerAccountNotFound]:
        "The block's issuer account could not be found.",
    [BlockFailureReason.VersionInvalid]:
        "The block's protocol version is invalid.",
    [BlockFailureReason.ManaCostCalculationFailed]:
        'The mana cost could not be calculated.',
    [BlockFailureReason.BurnedInsufficientMana]:
        "The block's issuer account burned insufficient Mana for a block.",
    [BlockFailureReason.AccountInvalid]: 'The account is invalid.',
    [BlockFailureReason.SignatureInvalid]: "The block's signature is invalid.",
    [BlockFailureReason.DroppedDueToCongestion]:
        'The block is dropped due to congestion.',
    [BlockFailureReason.PayloadInvalid]: 'The block payload is invalid.',
    [BlockFailureReason.Invalid]: 'The block is invalid.',
};
