# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from dataclasses import dataclass, field
from enum import IntEnum
from typing import Any, Dict, List, Union
from iota_sdk.types.common import HexStr, json


class ContextInputType(IntEnum):
    """Context input types.
    """
    Commitment = 0
    BlockIssuanceCredit = 1
    Reward = 2


@json
@dataclass
class ContextInput():
    """Base class for context inputs.
    """
    type: int


@json
@dataclass
class CommitmentContextInput(ContextInput):
    """A Commitment Context Input allows referencing a commitment to a certain slot.

    Attributes:
        type: The type of context input.
        commitment_id: The commitment identifier to reference.
    """
    commitment_id: HexStr
    type: int = field(
        default_factory=lambda: int(
            ContextInputType.Commitment),
        init=False)


@json
@dataclass
class BlockIssuanceCreditContextInput(ContextInput):
    """A Block Issuance Credit (BIC) Context Input provides the VM with context for the value of
    the BIC vector of a specific slot.

    Attributes:
        type: The type of context input.
        account_id: The ID of the Account for which this input provides the BIC.
    """
    account_id: HexStr
    type: int = field(
        default_factory=lambda: int(
            ContextInputType.BlockIssuanceCredit),
        init=False)


@json
@dataclass
class RewardContextInput(ContextInput):
    """A Reward Context Input indicates which transaction Input is claiming Mana rewards.

    Attributes:
        type: The type of context input.
        index: The index of the transaction input for which to claim rewards.
    """
    index: int
    type: int = field(
        default_factory=lambda: int(
            ContextInputType.Reward),
        init=False)


def context_input_from_dict(dict: Dict[str, Any]) -> Union[CommitmentContextInput,
                                                           BlockIssuanceCreditContextInput, RewardContextInput]:
    """
    Takes a dictionary as input and returns an instance of a specific class based on the value of the 'type' key in the dictionary.

    Arguments:
    * `dict`: A dictionary that is expected to have a key called 'type' which specifies the type of the returned value.
    """
    type = dict['type']
    if type == ContextInputType.Commitment:
        return CommitmentContextInput.from_dict(dict)
    if type == ContextInputType.BlockIssuanceCredit:
        return BlockIssuanceCreditContextInput.from_dict(dict)
    if type == ContextInputType.Reward:
        return RewardContextInput.from_dict(dict)
    raise Exception(f'invalid context input type: {type}')


def context_inputs_from_dicts(
        dicts: List[Dict[str, Any]]) -> List[Union[CommitmentContextInput, BlockIssuanceCreditContextInput, RewardContextInput]]:
    """
    Takes a list of dictionaries as input and returns a list with specific instances of a classes based on the value of the 'type' key in the dictionary.

    Arguments:
    * `dicts`: A list of dictionaries that are expected to have a key called 'type' which specifies the type of the returned value.
    """
    return list(map(context_input_from_dict, dicts))
