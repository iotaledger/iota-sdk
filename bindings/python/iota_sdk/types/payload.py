# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from iota_sdk.types.common import HexStr
from iota_sdk.types.signature import Ed25519Signature
from dataclasses import dataclass
from enum import Enum
from typing import Any, Optional, List


class PayloadType(Enum):
    """Block payload types.

    Attributes:
        TreasuryTransaction (4): A treasury transaction payload.
        TaggedData (5): A tagged data payload.
        Transaction (6): A transaction payload.
        Milestone (7): A milestone payload.
    """
    TreasuryTransaction = 4
    TaggedData = 5
    Transaction = 6
    Milestone = 7


class Payload():
    """Base class for `Block` payloads.

    Attributes:
        type: The type of payload.
        milestone: A `MilestonePayload` object if it represents a milestone payload.
        tagged_data: A `TaggedData` object if it represents a tagged data payload.
        transaction: A `Transaction` object if it represents a transaction payload.
        treasury_transaction: A `TreasuryTransaction` object if it represents a treasury transaction payload.

    """

    def __init__(self, type, milestone: Optional[Any] = None, tagged_data=None,
                 transaction=None, treasury_transaction: Optional[Any] = None):
        """Initialize a payload.
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
    """A milestone payload.

    Attributes:
        index: The index of corresponding milestone.
        timestamp: The timestamp of the corresponding milestone.
        protocolVersion: The current protocol version.
        previousMilestoneId: The ID of the previous milestone.
        parents: The parents of the milestone.
        inclusionMerkleRoot: The merkle root of all blocks included in the milestone cone.
        appliedMerkleRoot: The merkle root of all applied transactions in the milestone cone.
        signatures: The signatures that verify the milestone.
        options: The milestone options (e.g. receipt milestone option).
        metadata: Some hex encoded milestone metadata.
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
    """A tagged data payload.

    Attributes:
        tag: The tag part of the tagged data payload.
        data: The data part of the tagged data payload.
    """

    def __init__(self, tag: HexStr, data: HexStr):
        """Initialize a tagged data payload.
        """
        self.tag = tag
        self.data = data
        super().__init__(PayloadType.TaggedData, tagged_data=self)


class TransactionPayload(Payload):
    """A transaction payload.

    Attributes:
        essence: The transaction essence.
        unlocks: The unlocks of the transaction.
    """

    def __init__(self, essence, unlocks):
        """Initialize a transaction payload.
        """
        self.essence = essence
        self.unlocks = unlocks
        super().__init__(PayloadType.Transaction, transaction=self)
