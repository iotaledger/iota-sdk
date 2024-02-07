# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from enum import Enum, IntEnum
from dataclasses import dataclass
from typing import Optional
from iota_sdk.types.common import HexStr, json
from iota_sdk.types.block.block import Block
from iota_sdk.types.transaction_metadata import TransactionMetadata


@json
@dataclass
class BlockMetadata:
    """The metadata of a block.
    Response of GET /api/core/v3/blocks/{blockId}/metadata.

    Attributes:
        block_id: The identifier of the block. Hex-encoded with 0x prefix.
        block_state: If pending, the block is stored but not confirmed. If confirmed, the block is confirmed with the first level of knowledge. If finalized, the block is included and cannot be reverted anymore. If rejected, the block is rejected by the node, and user should reissue payload if it contains one. If failed, the block is not successfully issued due to failure reason.
        block_failure_reason: The optional block failure reason.
        transaction_metadata: The optional metadata of a given transaction.
    """
    block_id: HexStr
    block_state: BlockState
    block_failure_reason: Optional[BlockFailureReason] = None
    transaction_metadata: Optional[TransactionMetadata] = None


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
        ParentInvalid (4): One of the block's parents is invalid.
        IssuerAccountNotFound (5): The block's issuer account could not be found.
        VersionInvalid (6): The block's protocol version is invalid.
        ManaCostCalculationFailed (7): The mana cost could not be calculated.
        BurnedInsufficientMana (8): The block's issuer account burned insufficient Mana for a block.
        AccountInvalid (9): The account is invalid.
        SignatureInvalid (10): The block's signature is invalid.
        DroppedDueToCongestion (11): The block is dropped due to congestion.
        PayloadInvalid (12): The block payload is invalid.
        Invalid (255): The block is invalid.
    """
    TooOldToIssue = 1
    ParentTooOld = 2
    ParentDoesNotExist = 3
    ParentInvalid = 4
    IssuerAccountNotFound = 5
    VersionInvalid = 6
    ManaCostCalculationFailed = 7
    BurnedInsufficientMana = 8
    AccountInvalid = 9
    SignatureInvalid = 10
    DroppedDueToCongestion = 11
    PayloadInvalid = 12
    Invalid = 255

    def __str__(self):
        return {
            1: "The block is too old to issue.",
            2: "One of the block's parents is too old.",
            3: "One of the block's parents does not exist.",
            4: "One of the block's parents is invalid.",
            5: "The block's issuer account could not be found.",
            6: "The block's protocol version is invalid.",
            7: "The mana cost could not be calculated.",
            8: "The block's issuer account burned insufficient Mana for a block.",
            9: "The account is invalid.",
            10: "The block's signature is invalid.",
            11: "The block is dropped due to congestion.",
            12: "The block payload is invalid.",
            255: "The block is invalid."
        }[self.value]


@json
@dataclass
class BlockWithMetadata:
    """A block and its metadata.
    Response of GET /api/core/v3/blocks/{blockId}/full.

    Attributes:
        block: The block.
        metadata: The block metadata.
    """
    block: Block
    metadata: BlockMetadata
