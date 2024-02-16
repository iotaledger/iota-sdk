# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from enum import Enum, IntEnum
from dataclasses import dataclass
from typing import Optional

from iota_sdk.types.common import json
from iota_sdk.types.block.id import BlockId
from iota_sdk.types.transaction_metadata import TransactionFailureReason, TransactionState


@json
@dataclass
class BlockMetadata:
    """Represents the metadata of a block.
    Response of GET /api/core/v3/blocks/{blockId}/metadata.

    Attributes:
        block_state: The block state.
        transaction_state: The transaction state.
        block_failure_reason: The block failure reason.
        transaction_failure_reason: The transaction failure reason.
    """
    block_id: BlockId
    block_state: BlockState
    transaction_state: Optional[TransactionState] = None
    block_failure_reason: Optional[BlockFailureReason] = None
    transaction_failure_reason: Optional[TransactionFailureReason] = None


class BlockState(Enum):
    """Describes the state of a block.

    Attributes:
        Pending: Stored but not accepted/confirmed.
        Accepted: Valid block referenced by some validators.
        Confirmed: Valid block referenced by more than 2/3 of the validators.
        Finalized: Accepted/confirmed block and the slot was finalized, can no longer be reverted.
        Rejected: Rejected by the node, and user should reissue payload if it contains one.
        Failed: Not successfully issued due to failure reason.
    """
    Pending = 0
    Accepted = 1
    Confirmed = 2
    Finalized = 3
    Rejected = 4
    Failed = 5


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
