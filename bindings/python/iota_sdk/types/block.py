# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from enum import Enum
from dataclasses import dataclass
from typing import List, Optional
from iota_sdk.types.common import HexStr, json
from iota_sdk.types.payload import TaggedDataPayload, TransactionPayload, MilestonePayload
from iota_sdk.utils import Utils


@json
@dataclass
class Block:
    """Represent the object that nodes gossip around the network.

    Attributes:
        protocol_version: The protocol version with which this block was issued.
        parents: The parents of this block.
        burned_mana: The amount of Mana the Account identified by the IssuerId is at most willing to burn for this block.
        payload: The optional payload of this block.
    """

    protocol_version: int
    parents: List[HexStr]
    burned_mana: str
    payload: Optional[TaggedDataPayload |
                      TransactionPayload | MilestonePayload] = None

    def id(self) -> HexStr:
        return Utils.block_id(self)


class LedgerInclusionState(str, Enum):
    """Represents whether a block is included in the ledger.

    Attributes:
        noTransaction: The block does not contain a transaction.
        included: The block contains an included transaction.
        conflicting: The block contains a conflicting transaction.
    """
    noTransaction = 'noTransaction'
    included = 'included'
    conflicting = 'conflicting'


class ConflictReason(Enum):
    """Represents the possible reasons for a conflicting transaction.

    Attributes:
        none (0): The transaction does not conflict with the ledger.
        inputUTXOAlreadySpent (1): The input UTXO is already spent.
        inputUTXOAlreadySpentInThisMilestone (2): The input UTXO is already spent in this milestone.
        inputUTXONotFound (3): The input UTXO was not found.
        inputOutputSumMismatch (4): The sum of input and output amounts is not equal.
        invalidSignature (5): The signature is invalid.
        invalidTimelock (6): The timelock is invalid.
        invalidNativeTokens (7): The native tokens are invalid.
        returnAmountMismatch (8): The return amount is invalid.
        invalidInputUnlock (9): Not all inputs can be unlocked.
        invalidInputsCommitment (10): The inputs commitment hash is invalid.
        invalidSender (11): The sender is invalid.
        invalidChainState (12): The chain state is invalid.
        semanticValidationFailed (255): The semantic validation failed.
    """
    none = 0,
    inputUTXOAlreadySpent = 1,
    inputUTXOAlreadySpentInThisMilestone = 2,
    inputUTXONotFound = 3,
    inputOutputSumMismatch = 4,
    invalidSignature = 5,
    invalidTimelock = 6,
    invalidNativeTokens = 7,
    returnAmountMismatch = 8,
    invalidInputUnlock = 9,
    invalidInputsCommitment = 10,
    invalidSender = 11,
    invalidChainState = 12,
    semanticValidationFailed = 255,


@json
@dataclass
class BlockMetadata:
    """Block Metadata.

    Attributes:
        block_id: The id of the block.
        parents: The parents of the block.
        is_solid: Whether the block is solid.
        referenced_by_milestone_index: The milestone index referencing the block.
        milestone_index: The milestone index if the block contains a milestone payload.
        ledger_inclusion_state: The ledger inclusion state of the block.
        conflict_reason: The optional conflict reason of the block.
        should_promote: Whether the block should be promoted.
        should_reattach: Whether the block should be reattached.
    """
    block_id: HexStr
    parents: List[HexStr]
    is_solid: bool
    referenced_by_milestone_index: Optional[int] = None
    milestone_index: Optional[int] = None
    ledger_inclusion_state: Optional[LedgerInclusionState] = None
    conflict_reason: Optional[ConflictReason] = None
    should_promote: Optional[bool] = None
    should_reattach: Optional[bool] = None
