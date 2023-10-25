# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from dataclasses import dataclass
from typing import Dict, List, Optional
from enum import Enum
from iota_sdk.types.common import HexStr
from iota_sdk.types.output import OutputWithMetadata
from iota_sdk.types.payload import TransactionPayload


class InclusionState(str, Enum):
    """Inclusion state variants of a transaction.

    Attributes:
        Pending: The transaction is pending.
        Confirmed: The transaction is confirmed.
        Conflicting: The transaction is conflicting.
        UnknownPruned: The transaction is unknown or already pruned.
    """
    Pending = 'pending'
    Confirmed = 'confirmed'
    Conflicting = 'conflicting'
    UnknownPruned = 'unknownPruned'


@dataclass
class Transaction:
    """A transaction with some metadata.

    Attributes:
        payload: The transaction payload.
        inclusionState: The inclusion state of the transaction.
        timestamp: The timestamp of the transaction.
        transactionId: The ID of the corresponding transaction.
        networkId: The ID of the network this transaction was issued in.
        incoming: Indicates whether the transaction was created by the wallet or whether it was sent by someone else and is incoming.
        inputs: The inputs of the transaction.
        note: A note attached to the transaction.
        blockId: The ID of the block that holds the transaction.
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

    # pylint: disable=redefined-builtin
    @classmethod
    def from_dict(cls, dict: Dict) -> Transaction:
        """Converts a dict to a Transaction
        """
        obj = cls.__new__(cls)
        super(Transaction, obj).__init__()
        for k, v in dict.items():
            setattr(obj, k, v)
        return obj
