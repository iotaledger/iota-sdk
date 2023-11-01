# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from enum import IntEnum
from typing import Any, Dict, List, TypeAlias, Union
from dataclasses import dataclass, field
from iota_sdk.types.common import HexStr, json
from bindings.python.iota_sdk.types.transaction import Transaction
from iota_sdk.types.unlock import SignatureUnlock, ReferenceUnlock


class PayloadType(IntEnum):
    """Block payload types.

    Attributes:
        TaggedData (0): A tagged data payload.
        SignedTransaction (1): A signed transaction payload.
        CandidacyAnnouncement (2): A candidacy announcement payload.
    """
    TaggedData = 0
    SignedTransaction = 1
    CandidacyAnnouncement = 2


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
class SignedTransactionPayload:
    """A signed transaction payload.

    Attributes:
        transaction: The transaction.
        unlocks: The unlocks of the transaction.
    """
    transaction: Transaction
    unlocks: List[Union[SignatureUnlock, ReferenceUnlock]]
    type: int = field(
        default_factory=lambda: int(
            PayloadType.SignedTransaction),
        init=False)


@json
@dataclass
class CandidacyAnnouncementPayload:
    """A payload which is used to indicate candidacy for committee selection for the next epoch.
    """
    type: int = field(
        default_factory=lambda: int(
            PayloadType.CandidacyAnnouncement),
        init=False)


Payload: TypeAlias = Union[TaggedDataPayload,
                           SignedTransactionPayload, CandidacyAnnouncementPayload]


def deserialize_payload(d: Dict[str, Any]) -> Payload:
    """
    Takes a dictionary as input and returns an instance of a specific class based on the value of the 'type' key in the dictionary.

    Arguments:
    * `d`: A dictionary that is expected to have a key called 'type' which specifies the type of the returned value.
    """
    payload_type = d['type']
    if payload_type == PayloadType.TaggedData:
        return TaggedDataPayload.from_dict(d)
    if payload_type == PayloadType.SignedTransaction:
        return SignedTransactionPayload.from_dict(d)
    if payload_type == PayloadType.CandidacyAnnouncement:
        return CandidacyAnnouncementPayload.from_dict(d)
    raise Exception(f'invalid payload type: {payload_type}')


def deserialize_payloads(
        dicts: List[Dict[str, Any]]) -> List[Payload]:
    """
    Takes a list of dictionaries as input and returns a list with specific instances of classes based on the value of the 'type' key in the dictionary.

    Arguments:
    * `dicts`: A list of dictionaries that are expected to have a key called 'type' which specifies the type of the returned value.
    """
    return list(map(deserialize_payload, dicts))
