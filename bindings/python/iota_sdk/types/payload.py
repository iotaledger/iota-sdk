# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from dataclasses import dataclass, field
from enum import IntEnum
from typing import Any, Optional, List, Union
from dacite import from_dict
from iota_sdk.types.common import HexStr
from iota_sdk.types.output import BasicOutput, AliasOutput, FoundryOutput, NftOutput
from iota_sdk.types.input import UtxoInput
from iota_sdk.types.signature import Ed25519Signature
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


@dataclass
class TransactionEssence:
    """ Base essence class
    """
    type: int


@dataclass
class RegularTransactionEssence(TransactionEssence):
    """Describes the essence data making up a transaction by defining its inputs, outputs, and an optional payload.

    Attributes:
        networkId: The unique value denoting whether the block was meant for mainnet, shimmer, testnet, or a private network.
        inputsCommitment: BLAKE2b-256 hash serving as a commitment to the serialized outputs referenced by Inputs.
        inputs: The inputs to consume in order to fund the outputs of the Transaction Payload.
        outputs: The outputs that are created by the Transaction Payload
        payload: An optional tagged data payload
    """
    networkId: str
    inputsCommitment: HexStr
    inputs: List[UtxoInput]
    outputs: List[Union[AliasOutput, FoundryOutput, NftOutput, BasicOutput]]
    payload: Optional[TaggedDataPayload] = None
    type: int = field(default_factory=lambda: 1, init=False)

    def as_dict(self):
        """Converts this object to a dict.
        """
        config = {k: v for k, v in self.__dict__.items() if v is not None}

        if 'payload' in config:
            config['payload'] = config['payload'].as_dict()

        config['inputs'] = list(map(
            lambda x: x.__dict__, config['inputs']))

        config['outputs'] = list(map(
            lambda x: x.as_dict(), config['outputs']))

        return config


@dataclass
class Payload():
    """Initialize a Payload.
    """
    type: int

    def as_dict(self):
        """Converts this object to a dict.
        """
        config = {k: v for k, v in self.__dict__.items() if v is not None}

        if 'essence' in config:
            config['essence'] = config['essence'].as_dict()
        if 'unlocks' in config:
            def convert_to_dict(c):
                try:
                    return c.as_dict()
                except AttributeError:
                    return c.__dict__
            config['unlocks'] = list(map(convert_to_dict, config['unlocks']))
        if 'signatures' in config:
            config['signatures'] = list(map(
                lambda x: x.__dict__, config['signatures']))

        return config


@dataclass
class MilestonePayload(Payload):
    """A milestone payload.

    Attributes:
        index: The index of corresponding milestone.
        timestamp: The timestamp of the corresponding milestone.
        protocolVersion: The current protocol version.
        previousMilestoneId: The ID of the previous milestone.
        parents: The parents of the milestone.
        inclusionMerkleRoot: The merkle root of all blocks included in the milestone cone.
        appliedMerkleRoot: The merkle root of all applied transactions in the milestone cone.
        signatures: The signatures that verify the milestone.
        options: The milestone options (e.g. receipt milestone option).
        metadata: Some hex encoded milestone metadata.
    """
    index: int
    timestamp: int
    protocolVersion: int
    previousMilestoneId: HexStr
    parents: List[HexStr]
    inclusionMerkleRoot: HexStr
    appliedMerkleRoot: HexStr
    signatures: List[Ed25519Signature]
    options: Optional[List[Any]] = None
    metadata: Optional[HexStr] = None
    type: int = field(
        default_factory=lambda: int(
            PayloadType.Milestone),
        init=False)

    @classmethod
    def from_dict(cls, milestone_dict) -> MilestonePayload:
        """Converts a dict to a MilestonePayload
        """
        return from_dict(MilestonePayload, milestone_dict)


@dataclass
class TaggedDataPayload(Payload):
    """A tagged data payload.

    Attributes:
        tag: The tag part of the tagged data payload.
        data: The data part of the tagged data payload.
    """
    tag: Optional[HexStr] = None
    data: Optional[HexStr] = None
    type: int = field(
        default_factory=lambda: int(
            PayloadType.TaggedData),
        init=False)


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
