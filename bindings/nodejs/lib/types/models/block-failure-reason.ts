// Copyright 2023 IOTA Stiftung
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
    // The block's issuer account burned insufficient Mana for a block.
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
