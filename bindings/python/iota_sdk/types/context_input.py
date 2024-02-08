# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from dataclasses import dataclass, field
from enum import IntEnum
from typing import Any, Dict, List, TypeAlias, Union
from iota_sdk.types.common import HexStr, json
from iota_sdk.types.slot import SlotCommitmentId


class ContextInputType(IntEnum):
    """Context input types.
    """
    Commitment = 0
    BlockIssuanceCredit = 1
    Reward = 2


@json
@dataclass
class CommitmentContextInput:
    """A Commitment Context Input allows referencing a commitment to a certain slot.

    Attributes:
        type: The type of context input.
        commitment_id: The commitment identifier to reference.
    """
    type: int = field(
        default_factory=lambda: int(
            ContextInputType.Commitment),
        init=False)
    commitment_id: SlotCommitmentId


@json
@dataclass
class BlockIssuanceCreditContextInput:
    """A Block Issuance Credit (BIC) Context Input provides the VM with context for the value of
    the BIC vector of a specific slot.

    Attributes:
        type: The type of context input.
        account_id: The ID of the Account for which this input provides the BIC.
    """
    type: int = field(
        default_factory=lambda: int(
            ContextInputType.BlockIssuanceCredit),
        init=False)
    account_id: HexStr


@json
@dataclass
class RewardContextInput:
    """A Reward Context Input indicates which transaction Input is claiming Mana rewards.

    Attributes:
        type: The type of context input.
        index: The index of the transaction input for which to claim rewards.
    """
    type: int = field(
        default_factory=lambda: int(
            ContextInputType.Reward),
        init=False)
    index: int


ContextInput: TypeAlias = Union[CommitmentContextInput,
                                BlockIssuanceCreditContextInput, RewardContextInput]


def deserialize_context_input(d: Dict[str, Any]) -> ContextInput:
    """
    Takes a dictionary as input and returns an instance of a specific class based on the value of the 'type' key in the dictionary.

    Arguments:
    * `d`: A dictionary that is expected to have a key called 'type' which specifies the type of the returned value.
    """
    context_input_type = d['type']
    if context_input_type == ContextInputType.Commitment:
        return CommitmentContextInput.from_dict(d)
    if context_input_type == ContextInputType.BlockIssuanceCredit:
        return BlockIssuanceCreditContextInput.from_dict(d)
    if context_input_type == ContextInputType.Reward:
        return RewardContextInput.from_dict(d)
    raise Exception(f'invalid context input type: {context_input_type}')


def deserialize_context_inputs(
        dicts: List[Dict[str, Any]]) -> List[ContextInput]:
    """
    Takes a list of dictionaries as input and returns a list with specific instances of classes based on the value of the 'type' key in the dictionary.

    Arguments:
    * `dicts`: A list of dictionaries that are expected to have a key called 'type' which specifies the type of the returned value.
    """
    return list(map(deserialize_context_input, dicts))
