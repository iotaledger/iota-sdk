# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from enum import IntEnum
from typing import Any, Dict, List, Optional, TypeAlias, Union
from dataclasses import dataclass, field
from dataclasses_json import config
from iota_sdk.types.common import HexStr, json, SlotIndex
from iota_sdk.types.mana import ManaAllotment
from iota_sdk.types.input import UtxoInput, deserialize_inputs
from iota_sdk.types.context_input import ContextInput, deserialize_context_inputs
from iota_sdk.types.output import Output, deserialize_outputs
from iota_sdk.types.unlock import Unlock, deserialize_unlocks


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
    type: int = field(
        default_factory=lambda: int(
            PayloadType.TaggedData),
        init=False)
    tag: Optional[HexStr] = None
    data: Optional[HexStr] = None


def deserialize_payload(d: Dict[str, Any]) -> Payload:
    """Takes a dictionary as input and returns an instance of a specific class based on the value of the 'type' key in the dictionary.

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
    """Takes a list of dictionaries as input and returns a list with specific instances of classes based on the value of the 'type' key in the dictionary.

    Arguments:
    * `dicts`: A list of dictionaries that are expected to have a key called 'type' which specifies the type of the returned value.
    """
    return list(map(deserialize_payload, dicts))


def deserialize_tagged_data_payload(d: Dict[str, Any]) -> Payload:
    """Takes a dictionary as input and returns an instance of a specific class based on the value of the 'type' key in the dictionary.

    Arguments:
    * `d`: A dictionary that is expected to have a key called 'type' which specifies the type of the returned value.
    """
    payload_type = d['type']
    if payload_type == PayloadType.TaggedData:
        return TaggedDataPayload.from_dict(d)
    raise Exception(f'invalid payload type: {payload_type}')


@json
@dataclass
class Transaction:
    """A transaction consuming inputs, creating outputs and carrying an optional payload.

    Attributes:
        network_id: The unique value denoting whether the block was meant for mainnet, shimmer, testnet, or a private network.
                    It consists of the first 8 bytes of the BLAKE2b-256 hash of the network name.
        creation_slot: The slot index in which the transaction was created.
        context_inputs: The inputs that provide additional contextual information for the execution of a transaction.
        inputs: The inputs to consume in order to fund the outputs of the Transaction Payload.
        allotments: The allotments of Mana which which will be added upon commitment of the slot.
        capabilities: The capability bitflags of the transaction.
        outputs: The outputs that are created by the Transaction Payload
        payload: An optional tagged data payload
    """
    network_id: str
    creation_slot: SlotIndex
    inputs: List[UtxoInput] = field(metadata=config(
        decoder=deserialize_inputs
    ))
    outputs: List[Output] = field(metadata=config(
        decoder=deserialize_outputs
    ))
    capabilities: Optional[HexStr] = None
    context_inputs: Optional[List[ContextInput]] = field(default=None, metadata=config(
        decoder=deserialize_context_inputs
    ))
    allotments: Optional[List[ManaAllotment]] = None
    payload: Optional[Payload] = field(default=None, metadata=config(
        decoder=deserialize_tagged_data_payload
    ))

    def with_capabilities(self, capabilities: bytes):
        """Sets the transaction capabilities from a byte array.
        Attributes:
            capabilities: The transaction capabilities bitflags.
        """
        if any(c != 0 for c in capabilities):
            self.capabilities = '0x' + capabilities.hex()
        else:
            self.capabilities = None


@json
@dataclass
class SignedTransactionPayload:
    """A signed transaction payload.

    Attributes:
        transaction: The transaction.
        unlocks: The unlocks of the transaction.
    """
    type: int = field(
        default_factory=lambda: int(
            PayloadType.SignedTransaction),
        init=False)
    transaction: Transaction
    unlocks: List[Unlock] = field(metadata=config(
        decoder=deserialize_unlocks
    ))


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
