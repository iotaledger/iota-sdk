# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from enum import IntEnum
from typing import Any, Dict, List, Union

from dataclasses import dataclass, field

from iota_sdk.types.common import HexStr, json
from iota_sdk.types.essence import RegularTransactionEssence
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


def payload_from_dict(dict: Dict[str, Any]) -> Union[TaggedDataPayload, TransactionPayload]:
    """
    Takes a dictionary as input and returns an instance of a specific class based on the value of the 'type' key in the dictionary.

    Arguments:
    * `dict`: A dictionary that is expected to have a key called 'type' which specifies the type of the returned value.
    """
    type = dict['type']
    if type == PayloadType.TaggedData:
        return TaggedDataPayload.from_dict(dict)
    if type == PayloadType.Transaction:
        return TransactionPayload.from_dict(dict)
    raise Exception(f'invalid payload type: {type}')


def payloads_from_dicts(
        dicts: List[Dict[str, Any]]) -> List[Union[TaggedDataPayload, TransactionPayload]]:
    """
    Takes a list of dictionaries as input and returns a list with specific instances of a classes based on the value of the 'type' key in the dictionary.

    Arguments:
    * `dicts`: A list of dictionaries that are expected to have a key called 'type' which specifies the type of the returned value.
    """
    return list(map(payload_from_dict, dicts))
