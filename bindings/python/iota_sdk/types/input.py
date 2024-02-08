# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from enum import IntEnum
from typing import Dict, List, TypeAlias, Union, Any
from dataclasses import dataclass, field
from iota_sdk.types.common import json
from iota_sdk.types.transaction_id import TransactionId


class InputType(IntEnum):
    """Input types.

    Attributes:
        Utxo: An unspent transaction output.
    """
    Utxo = 0


@json
@dataclass
class UtxoInput:
    """Represents an input referencing an output.

    Attributes:
        type: The type of input.
        transaction_id: The transaction id that created the output.
        transaction_output_index: The output index.
    """
    type: int = field(
        default_factory=lambda: int(
            InputType.Utxo),
        init=False)
    transaction_id: TransactionId
    transaction_output_index: int


Input: TypeAlias = Union[UtxoInput]


def deserialize_input(d: Dict[str, Any]) -> Input:
    """
    Takes a dictionary as input and returns an instance of a specific class based on the value of the 'type' key in the dictionary.

    Arguments:
    * `d`: A dictionary that is expected to have a key called 'type' which specifies the type of the returned value.
    """
    input_type = d['type']
    if input_type == InputType.Utxo:
        return UtxoInput.from_dict(d)
    raise Exception(f'invalid input type: {input_type}')


def deserialize_inputs(dicts: List[Dict[str, Any]]) -> List[Input]:
    """
    Takes a list of dictionaries as input and returns a list with specific instances of classes based on the value of the 'type' key in the dictionary.

    Arguments:
    * `dicts`: A list of dictionaries that are expected to have a key called 'type' which specifies the type of the returned value.
    """
    return list(map(deserialize_input, dicts))
