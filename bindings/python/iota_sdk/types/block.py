# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from dataclasses import dataclass
from typing import Dict, List, Optional
from iota_sdk.types.common import HexStr
from iota_sdk.types.payload import Payload
from iota_sdk.utils import Utils
from enum import Enum


@dataclass
class Block:
    """Represent the object that nodes gossip around the network.
    """

    protocolVersion: int
    parents: List[HexStr]
    nonce: int
    payload: Optional[Payload] = None

    @classmethod
    def from_dict(cls, block_dict: Dict) -> Block:
        obj = cls.__new__(cls)
        super(Block, obj).__init__()
        for k, v in block_dict.items():
            setattr(obj, k, v)
        return obj

    def id(self) -> HexStr:
        return Utils.block_id(self)


class LedgerInclusionState(str, Enum):
    noTransaction = 'noTransaction'
    included = 'included'
    conflicting = 'conflicting'


class ConflictReason(Enum):
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


@dataclass
class BlockMetadata:
    """Block Metadata.
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
        obj = cls.__new__(cls)
        super(BlockMetadata, obj).__init__()
        for k, v in block_metadata_dict.items():
            setattr(obj, k, v)
        return obj
