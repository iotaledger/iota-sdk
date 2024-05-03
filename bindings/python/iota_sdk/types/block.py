# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from dataclasses import dataclass
from typing import List, Optional, Union, Dict
from enum import Enum
from dacite import from_dict
from iota_sdk.types.common import HexStr
from iota_sdk.types.payload import TaggedDataPayload, TransactionPayload, MilestonePayload
from iota_sdk.utils import Utils


@dataclass
class Block:
    """Represent the object that nodes gossip around the network.

    Attributes:
        protocolVersion: The protocol version with which this block was issued.
        parents: The parents of this block.
        nonce: The nonce of this block.
        payload: The optional payload of this block.
    """

    protocolVersion: int
    parents: List[HexStr]
    nonce: str
    payload: Optional[Union[TransactionPayload,
                            MilestonePayload, TaggedDataPayload]] = None

    @classmethod
    def from_dict(cls, block_dict: Dict) -> Block:
        """
        Takes a dictionary that contains the data needed to
        create an instance of the `Block` class.

        Returns:

        An instance of the `Block` class.
        """
        return from_dict(Block, block_dict)

    def id(self) -> HexStr:
        """Returns the block ID as a hexadecimal string.
        """
        return Utils.block_id(self)

    def as_dict(self):
        """Converts this object to a dict.
        """
        config = {k: v for k, v in self.__dict__.items() if v is not None}

        if 'payload' in config:
            config['payload'] = config['payload'].as_dict()

        return config


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
    ConflictReason.semanticValidationFailed: 'The semantic validation failed'
}


@dataclass
class BlockMetadata:
    """Block Metadata.

    Attributes:
        blockId: The id of the block.
        parents: The parents of the block.
        isSolid: Whether the block is solid.
        referencedByMilestoneIndex: The milestone index referencing the block.
        milestoneIndex: The milestone index if the block contains a milestone payload.
        ledgerInclusionState: The ledger inclusion state of the block.
        conflictReason: The optional conflict reason of the block.
        shouldPromote: Whether the block should be promoted.
        shouldReattach: Whether the block should be reattached.
    """
    blockId: HexStr
    parents: List[HexStr]
    isSolid: bool
    referencedByMilestoneIndex: Optional[int] = None
    milestoneIndex: Optional[int] = None
    ledgerInclusionState: Optional[LedgerInclusionState] = None
    conflictReason: Optional[ConflictReason] = None
    shouldPromote: Optional[bool] = None
    shouldReattach: Optional[bool] = None

    @classmethod
    def from_dict(cls, block_metadata_dict: Dict) -> BlockMetadata:
        """Converts a dict to a BlockMetadata
        """
        obj = cls.__new__(cls)
        super(BlockMetadata, obj).__init__()
        for k, v in block_metadata_dict.items():
            setattr(obj, k, v)
        return obj
