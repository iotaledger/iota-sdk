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
    Pending = 'pending'
    Confirmed = 'confirmed'
    Conflicting = 'conflicting'
    UnknownPruned = 'unknownPruned'


@dataclass
class Transaction:
    """The transaction payload with metadata.
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
