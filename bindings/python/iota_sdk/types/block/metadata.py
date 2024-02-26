# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from enum import Enum, IntEnum


class BlockState(str, Enum):
    """Describes the state of a block.

    Attributes:
        Pending: Stored but not accepted/confirmed.
        Accepted: Valid block referenced by some validators.
        Confirmed: Valid block referenced by more than 2/3 of the validators.
        Finalized: Accepted/confirmed block and the slot was finalized, can no longer be reverted.
        Rejected: Rejected by the node, and user should reissue payload if it contains one.
        Failed: Not successfully issued due to failure reason.
    """
    Pending = 'pending'
    Accepted = 'accepted'
    Confirmed = 'confirmed'
    Finalized = 'finalized'
    Rejected = 'rejected'
    Failed = 'failed'


class BlockFailureReason(IntEnum):
    """Describes the reason of a block failure.

    Attributes:
        TooOldToIssue (1): The block is too old to issue.
        ParentTooOld (2): One of the block's parents is too old.
        ParentDoesNotExist (3): One of the block's parents does not exist.
        IssuerAccountNotFound (4): The block's issuer account could not be found.
        ManaCostCalculationFailed (5): The mana cost could not be calculated.
        BurnedInsufficientMana (6): The block's issuer account burned insufficient Mana for a block.
        AccountLocked (7): The account is locked.
        AccountExpired (8): The account is expired.
        SignatureInvalid (9): The block's signature is invalid.
        DroppedDueToCongestion (10): The block is dropped due to congestion.
        PayloadInvalid (11): The block payload is invalid.
        Invalid (255): The block is invalid.
    """
    TooOldToIssue = 1
    ParentTooOld = 2
    ParentDoesNotExist = 3
    IssuerAccountNotFound = 4
    ManaCostCalculationFailed = 5
    BurnedInsufficientMana = 6
    AccountLocked = 7
    AccountExpired = 8
    SignatureInvalid = 9
    DroppedDueToCongestion = 10
    PayloadInvalid = 11
    Invalid = 255

    def __str__(self):
        return {
            1: "The block is too old to issue.",
            2: "One of the block's parents is too old.",
            3: "One of the block's parents does not exist.",
            4: "The block's issuer account could not be found.",
            5: "The mana cost could not be calculated.",
            6: "The block's issuer account burned insufficient Mana for a block.",
            7: "The account is locked.",
            8: "The account is expired.",
            9: "The block's signature is invalid.",
            10: "The block is dropped due to congestion.",
            11: "The block payload is invalid.",
            255: "The block is invalid."
        }[self.value]
