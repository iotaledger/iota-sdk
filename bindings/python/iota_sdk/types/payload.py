# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from iota_sdk.types.common import HexStr
from iota_sdk.types.signature import Ed25519Signature
from dataclasses import dataclass
from enum import Enum
from typing import Any, Optional, List


class PayloadType(Enum):
    TreasuryTransaction = 4
    TaggedData = 5
    Transaction = 6
    Milestone = 7


class Payload():
    def __init__(self, type,  milestone: Optional[Any] = None, tagged_data=None, transaction=None, treasury_transaction: Optional[Any] = None):
        """Initialize a payload
        """
        self.type = type
        self.milestone = milestone
        self.tagged_data = tagged_data
        self.transaction = transaction
        self.treasury_transaction = treasury_transaction

    def as_dict(self):
        config = {k: v for k, v in self.__dict__.items() if v != None}

        if "milestone" in config:
            del config["milestone"]
        if "tagged_data" in config:
            del config["tagged_data"]
        if "transaction" in config:
            del config["transaction"]
        if "treasury_transaction" in config:
            del config["treasury_transaction"]

        config['type'] = config['type'].value

        return config


@dataclass
class MilestonePayload(Payload):
    """Initialize a MilestonePayload
    """
    index: int
    timestamp: int
    protocolVersion: int
    previousMilestoneId: HexStr
    parents: List[HexStr]
    inclusionMerkleRoot: HexStr
    appliedMerkleRoot: HexStr
    signatures: List[Ed25519Signature]
    options: Optional[List[Any]] = None
    metadata: Optional[HexStr] = None

    @classmethod
    def from_dict(cls, milestone) -> MilestonePayload:
        obj = cls.__new__(cls)
        super(MilestonePayload, obj).__init__(milestone["type"])
        del milestone["type"]
        for k, v in milestone.items():
            setattr(obj, k, v)
        return obj


class TaggedDataPayload(Payload):
    def __init__(self, tag: HexStr, data: HexStr):
        """Initialize a TaggedDataPayload
        """
        self.tag = tag
        self.data = data
        super().__init__(PayloadType.TaggedData, tagged_data=self)


class TransactionPayload(Payload):
    def __init__(self, essence, unlocks):
        """Initialize a TransactionPayload
        """
        self.essence = essence
        self.unlocks = unlocks
        super().__init__(PayloadType.Transaction, transaction=self)
