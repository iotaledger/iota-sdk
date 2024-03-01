# Copyright 2024 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from enum import IntEnum
from typing import Any, Dict, TypeAlias, Union
from dataclasses import dataclass, field
from dataclasses_json import config
from iota_sdk.types.common import HexStr, json, SlotIndex


class OutputCommitmentProofType(IntEnum):
    """OutputCommitmentProof types.

    Attributes:
        HashableNode (0): Denotes a HashableNode.
        LeafHash (1): Denotes a LeafHash.
        ValueHash (2): Denotes a ValueHash.
    """
    HashableNode = 0
    LeafHash = 1
    ValueHash = 2


def deserialize_proof(d: Dict[str, Any]) -> OutputCommitmentProof:
    """
    Takes a dictionary as input and returns an instance of a specific class based on the value of the 'type' key in the dictionary.

    Arguments:
    * `d`: A dictionary that is expected to have a key called 'type' which specifies the type of the returned value.
    """
    proof_type = d['type']
    if proof_type == OutputCommitmentProofType.HashableNode:
        return HashableNode.from_dict(d)
    if proof_type == OutputCommitmentProofType.LeafHash:
        return LeafHash.from_dict(d)
    if proof_type == OutputCommitmentProofType.ValueHash:
        return ValueHash.from_dict(d)
    raise Exception(f'invalid proof type: {proof_type}')


@json
@dataclass
class HashableNode:
    """Contains the hashes of the left and right children of a node in the OutputCommitmentProof tree.
    """
    type: int = field(default_factory=lambda: int(
        OutputCommitmentProofType.HashableNode), init=False)
    l: OutputCommitmentProof = field(metadata=config(
        decoder=deserialize_proof
    ))
    r: OutputCommitmentProof = field(metadata=config(
        decoder=deserialize_proof
    ))


@json
@dataclass
class LeafHash:
    """Contains the hash of a leaf in the OutputCommitmentProof tree.
    """
    type: int = field(default_factory=lambda: int(
        OutputCommitmentProofType.LeafHash), init=False)
    hash: HexStr


@json
@dataclass
class ValueHash:
    """Contains the hash of the value for which the OutputCommitmentProof is being computed.
    """
    type: int = field(default_factory=lambda: int(
        OutputCommitmentProofType.ValueHash), init=False)
    hash: HexStr


@json
@dataclass
class OutputIdProof:
    """The proof of the output identifier.

    Attributes:
        slot: The slot index of the output.
        output_index: The index of the output within the corresponding transaction.
        transaction_commitment: The commitment of the transaction that created the output. Hex-encoded with 0x prefix.
        output_commitment_proof: The proof of the output commitment.
    """
    slot: SlotIndex
    output_index: int
    transaction_commitment: HexStr
    output_commitment_proof: OutputCommitmentProof = field(metadata=config(
        decoder=deserialize_proof
    ))


OutputCommitmentProof: TypeAlias = Union[HashableNode, LeafHash, ValueHash]
