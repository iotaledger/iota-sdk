# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from enum import Enum
from dataclasses import dataclass
from typing import List, Optional, Union, Dict
from dacite import from_dict
from iota_sdk.types.common import HexStr, json
from iota_sdk.types.payload import TaggedDataPayload, TransactionPayload
from iota_sdk.utils import Utils


@json
@dataclass
class Block:
    """Represent the object that nodes gossip around the network.

    Attributes:
        protocol_version: The protocol version with which this block was issued.
        strong_parents: Blocks that are strongly directly approved.
        weak_parents: Blocks that are weakly directly approved.
        shallow_like_parents: Blocks that are directly referenced to adjust opinion.
        burned_mana: The amount of Mana the Account identified by the IssuerId is at most willing to burn for this block.
        payload: The optional payload of this block.
    """

    protocol_version: int
    strong_parents: List[HexStr]
    weak_parents: List[HexStr]
    shallow_like_parents: List[HexStr]
    burned_mana: str
    payload: Optional[Union[TaggedDataPayload,
                      TransactionPayload]] = None

    @classmethod
    def from_dict(cls, block_dict: Dict) -> Block:
        """
        The function `from_dict` takes a dictionary that contains the data needed to
        create an instance of the `Block` class.
        
        Returns:
        
        An instance of the `Block` class.
        """
        return from_dict(Block, block_dict)

    def id(self) -> HexStr:
        """Rreturns the block ID as a hexadecimal string.
        """
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
    none = 0
    inputUTXOAlreadySpent = 1
    inputUTXOAlreadySpentInThisMilestone = 2
    inputUTXONotFound = 3
    inputOutputSumMismatch = 4
    invalidSignature = 5
    invalidTimelock = 6
    invalidNativeTokens = 7
    returnAmountMismatch = 8
    invalidInputUnlock = 9
    invalidInputsCommitment = 10
    invalidSender = 11
    invalidChainState = 12
    semanticValidationFailed = 255


CONFLICT_REASON_STRINGS = {
    ConflictReason.none: 'The block has no conflict',
    ConflictReason.inputUTXOAlreadySpent: 'The referenced UTXO was already spent',
    ConflictReason.inputUTXOAlreadySpentInThisMilestone: 'The referenced UTXO was already spent while confirming this milestone',
    ConflictReason.inputUTXONotFound: 'The referenced UTXO cannot be found',
    ConflictReason.inputOutputSumMismatch: 'The sum of the inputs and output values does not match',
    ConflictReason.invalidSignature: 'The unlock block signature is invalid',
    ConflictReason.invalidTimelock: 'The configured timelock is not yet expired',
    ConflictReason.invalidNativeTokens: 'The native tokens are invalid',
    ConflictReason.returnAmountMismatch: 'The return amount in a transaction is not fulfilled by the output side',
    ConflictReason.invalidInputUnlock: 'The input unlock is invalid',
    ConflictReason.invalidInputsCommitment: 'The inputs commitment is invalid',
    ConflictReason.invalidSender: ' The output contains a Sender with an ident (address) which is not unlocked',
    ConflictReason.invalidChainState: 'The chain state transition is invalid',
    ConflictReason.semanticValidationFailed: 'The semantic validation failed'}


@json
@dataclass
class BlockMetadata:
    """Block Metadata.

    Attributes:
        block_id: The id of the block.
        strong_parents: Blocks that are strongly directly approved.
        weak_parents: Blocks that are weakly directly approved.
        shallow_like_parents: Blocks that are directly referenced to adjust opinion.
        is_solid: Whether the block is solid.
        referenced_by_milestone_index: The milestone index referencing the block.
        milestone_index: The milestone index if the block contains a milestone payload.
        ledger_inclusion_state: The ledger inclusion state of the block.
        conflict_reason: The optional conflict reason of the block.
        should_promote: Whether the block should be promoted.
        should_reattach: Whether the block should be reattached.
    """
    block_id: HexStr
    strong_parents: List[HexStr]
    weak_parents: List[HexStr]
    shallow_like_parents: List[HexStr]
    is_solid: bool
    referenced_by_milestone_index: Optional[int] = None
    milestone_index: Optional[int] = None
    ledger_inclusion_state: Optional[LedgerInclusionState] = None
    conflict_reason: Optional[ConflictReason] = None
    should_promote: Optional[bool] = None
    should_reattach: Optional[bool] = None
