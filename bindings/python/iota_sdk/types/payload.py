# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from enum import IntEnum
from typing import Optional, List, Union

from dataclasses import dataclass, field

from iota_sdk.types.common import HexStr, json
from iota_sdk.types.output import BasicOutput, AccountOutput, FoundryOutput, NftOutput
from iota_sdk.types.input import UtxoInput
from iota_sdk.types.unlock import SignatureUnlock, ReferenceUnlock
from iota_sdk.types.mana import ManaAllotment


class PayloadType(IntEnum):
    """Block payload types.

    Attributes:
        TaggedData (5): A tagged data payload.
        Transaction (6): A transaction payload.
    """
    TaggedData = 5
    Transaction = 6


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
    unlocks: List[Union[SignatureUnlock, ReferenceUnlock]]
    type: int = field(
        default_factory=lambda: int(
            PayloadType.Transaction),
        init=False)
