# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from enum import IntEnum
from typing import Optional, List

from dataclasses import dataclass, field

from iota_sdk.types.common import HexStr, json
from iota_sdk.types.output import BasicOutput, AliasOutput, FoundryOutput, NftOutput
from iota_sdk.types.input import UtxoInput
from iota_sdk.types.unlock import SignatureUnlock, ReferenceUnlock


class PayloadType(IntEnum):
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


@json
@dataclass
class TransactionEssence:
    type: int


@json
@dataclass
class RegularTransactionEssence(TransactionEssence):
    network_id: str
    inputs_commitment: HexStr
    inputs: List[UtxoInput]
    outputs: List[AliasOutput | FoundryOutput | NftOutput | BasicOutput]
    payload: Optional[TaggedDataPayload] = None
    type: int = field(default_factory=lambda: 1, init=False)


@json
@dataclass
class Payload():
    """Initialize a Payload.
    """
    type: int


@json
@dataclass
class TaggedDataPayload(Payload):
    """A tagged data payload.

    Attributes:
        tag: The tag part of the tagged data payload.
        data: The data part of the tagged data payload.
    """
    tag: HexStr
    data: HexStr
    type: int = field(
        default_factory=lambda: int(
            PayloadType.TaggedData),
        init=False)


@json
@dataclass
class TransactionPayload(Payload):
    """A transaction payload.

    Attributes:
        essence: The transaction essence.
        unlocks: The unlocks of the transaction.
    """
    essence: RegularTransactionEssence
    unlocks: List[SignatureUnlock | ReferenceUnlock]
    type: int = field(
        default_factory=lambda: int(
            PayloadType.Transaction),
        init=False)
