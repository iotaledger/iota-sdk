# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from iota_sdk.types.common import HexStr
from iota_sdk.types.signature import Ed25519Signature
from dataclasses import dataclass
from enum import Enum
from typing import Any, Optional, List


class PayloadType(Enum):
    """Type of block payload.

    Attributes:
        TreasuryTransaction (4): TreasuryTransaction payload
        TaggedData (5): TaggedData payload
        Transaction (6): Transaction payload
        Milestone (7): Milestone payload
    """
    TreasuryTransaction = 4
    TaggedData = 5
    Transaction = 6
    Milestone = 7


class Payload():
    """Base class for block payloads.

    Attributes:
        type (int): Payload type
        milestone (optional): Milestone payload
        tagged_data (optional): TaggedData payload
        transaction (optional): Transaction payload
        treasury_transaction (optional): TreasuryTransaction payload
        
    """
    def __init__(self, type, milestone=None, tagged_data=None, transaction=None, treasury_transaction=None):
        """Initialize a payload

        Args:
            type (int): Payload type
            milestone (optional): Milestone payload
            tagged_data (optional): TaggedData payload
            transaction (optional): Transaction payload
            treasury_transaction (optional): TreasuryTransaction payload
        """
        self.type = type
        self.milestone = milestone
        self.tagged_data = tagged_data
        self.transaction = transaction
        self.treasury_transaction = treasury_transaction

    def as_dict(self):
        config = {k: v for k, v in self.__dict__.items() if v is not None}

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
    """Represents a MilestonePayload. Inherits `Payload`.

    Attributes:
        index (int): corresponding milestone index
        timestamp (int): corresponding milestone timestamp
        protocolVersion (int): current protocol version
        previousMilestoneId (HexStr): previous milestone id
        parents (List[HexStr]): parents of the milestone 
        inclusionMerkleRoot (HexStr): the inclusion merkle root
        appliedMerkleRoot (HexStr): the applied merkle root
        signatures (List[Ed25519Signature]): milestone signatures
        options (List[Any], optional): milestone options
        metadata (HexStr, optional): milestone metadata
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
    """Represents a TaggedDataPayload. Inherits `Payload`.

    Attributes:
        tag (HexStr): tag as hex string
        data (HexStr): data as hex string
    """
    def __init__(self, tag: HexStr, data: HexStr):
        """Initialize a TaggedDataPayload

        Args:
            tag (HexStr): tag as hex string
            data (HexStr): data as hex string
        """
        self.tag = tag
        self.data = data
        super().__init__(PayloadType.TaggedData, tagged_data=self)


class TransactionPayload(Payload):
    """Represents a TransactionPayload. Inherits `Payload`.

    Attributes:
        essence (HexStr): transaction essence as hex string
        unlocks (List[HexStr]): transaction unlocks as a list of hex strings
    """
    def __init__(self, essence, unlocks):
        """Initialize a TransactionPayload

        Args:
            essence (HexStr): transaction essence as hex string
            unlocks (List[HexStr]): transaction unlocks as a list of hex strings
        """
        self.essence = essence
        self.unlocks = unlocks
        super().__init__(PayloadType.Transaction, transaction=self)
