# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from dataclasses import dataclass
from typing import Dict, List, Optional
from iota_sdk.types.common import HexStr
from iota_sdk.types.output import OutputWithMetadata
from iota_sdk.types.payload import TransactionPayload
from enum import Enum


class InclusionState(str, Enum):
    """The inclusion state of a transaction.

    Attributes:
        Pending: The transaction is pending
        Confirmed: The transaction is confirmed
        Conflicting: The transaction is conflicting
        UnknownPruned: The transaction is unknowned or already pruned
    """
    Pending = 'pending'
    Confirmed = 'confirmed'
    Conflicting = 'conflicting'
    UnknownPruned = 'unknownPruned'


@dataclass
class Transaction:
    """Represents a transaction with metadata.

    Attributes:
        payload (TransactionPayload): the transaction payload
        inclusionState (InclusionState): inclusion state of the transaction
        timestamp (int): the timestamp of the transaction
        transactionId (HexStr): the corresponding transaction
        networkId (int): the network this transaction belongs to
        incoming (bool): whether the transaction is incoming
        inputs (List[OutputWithMetadata]): the inputs of the transaction
        note (str, optional): A note with the transaction
        blockId (HexStr, optional): the block that contains the transaction payload
    """
    payload: TransactionPayload
    inclusionState: InclusionState
    timestamp: int
    transactionId: HexStr
    networkId: int
    incoming: bool
    inputs = List[OutputWithMetadata]
    note: Optional[str] = None
    blockId: Optional[HexStr] = None

    @classmethod
    def from_dict(cls, dict: Dict) -> Transaction:
        obj = cls.__new__(cls)
        super(Transaction, obj).__init__()
        for k, v in dict.items():
            setattr(obj, k, v)
        return obj
