# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from iota_sdk.types.common import HexStr
from enum import Enum
from typing import Any, Optional


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


class MilestonePayload(Payload):
    def __init__(self, essence, signatures):
        """Initialize a MilestonePayload
        """
        self.essence = essence
        self.signatures = signatures
        super().__init__(PayloadType.Milestone, milestone=self)


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
