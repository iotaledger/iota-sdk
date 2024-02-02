# Copyright 2024 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from dataclasses import dataclass
from typing import List, Optional
from enum import Enum
from iota_sdk.types.common import HexStr, json
from iota_sdk.types.output_metadata import OutputWithMetadata
from iota_sdk.types.payload import SignedTransactionPayload


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


@json
@dataclass
class TransactionWithMetadata:
    """A transaction with some metadata.

    Attributes:
        payload: The transaction payload.
        inclusion_state: The inclusion state of the transaction.
        timestamp: The timestamp of the transaction.
        transaction_id: The ID of the corresponding transaction.
        network_id: The ID of the network this transaction was issued in.
        incoming: Indicates whether the transaction was created by the wallet or whether it was sent by someone else and is incoming.
        inputs: The inputs of the transaction.
        note: A note attached to the transaction.
        block_id: The ID of the block that holds the transaction.
    """
    payload: SignedTransactionPayload
    inclusion_state: InclusionState
    timestamp: int
    transaction_id: HexStr
    network_id: int
    incoming: bool
    inputs = List[OutputWithMetadata]
    note: Optional[str] = None
    block_id: Optional[HexStr] = None


@json
@dataclass
class CreateNativeTokenTransaction:
    """A transaction that created a native token.

    Attributes:
        token_id: The id of the created native token.
        transaction: The transaction that created the native token.
    """
    token_id: HexStr
    transaction: TransactionWithMetadata


@json
@dataclass
class CreateDelegationTransaction:
    """A transaction that created a delegation.

    Attributes:
        delegation_id: The id of the created delegation.
        transaction: The transaction that created the delegation.
    """
    delegation_id: HexStr
    transaction: TransactionWithMetadata
