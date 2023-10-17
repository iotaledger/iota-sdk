# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from enum import IntEnum
from typing import Any, Dict, List, TypeAlias, Union
from dataclasses import dataclass, field
from iota_sdk.types.common import HexStr, json
from iota_sdk.types.essence import TransactionEssence
from iota_sdk.types.unlock import SignatureUnlock, ReferenceUnlock


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
class TaggedDataPayload:
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
class TransactionPayload:
    """A transaction payload.

    Attributes:
        essence: The transaction essence.
        unlocks: The unlocks of the transaction.
    """
    essence: TransactionEssence
    unlocks: List[Union[SignatureUnlock, ReferenceUnlock]]
    type: int = field(
        default_factory=lambda: int(
            PayloadType.Transaction),
        init=False)


Payload: TypeAlias = Union[TaggedDataPayload, TransactionPayload]


def deserialize_payload(d: Dict[str, Any]) -> Payload:
    """
    Takes a dictionary as input and returns an instance of a specific class based on the value of the 'type' key in the dictionary.

    Arguments:
    * `d`: A dictionary that is expected to have a key called 'type' which specifies the type of the returned value.
    """
    payload_type = d['type']
    if payload_type == PayloadType.TaggedData:
        return TaggedDataPayload.from_dict(d)
    if payload_type == PayloadType.Transaction:
        return TransactionPayload.from_dict(d)
    raise Exception(f'invalid payload type: {payload_type}')


def deserialize_payloads(
        dicts: List[Dict[str, Any]]) -> List[Payload]:
    """
    Takes a list of dictionaries as input and returns a list with specific instances of classes based on the value of the 'type' key in the dictionary.

    Arguments:
    * `dicts`: A list of dictionaries that are expected to have a key called 'type' which specifies the type of the returned value.
    """
    return list(map(deserialize_payload, dicts))
