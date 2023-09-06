# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from dataclasses import dataclass
from enum import IntEnum
from iota_sdk.types.common import HexStr, json


class InputType(IntEnum):
    """Input types.

    Attributes:
        Utxo: An unspent transaction output.
        Treasury: The treasury output.
    """
    Utxo = 0
    Treasury = 1


@json
@dataclass
class UtxoInput:
    """Represents an input referencing an output.

    Attributes:
        type: The type of input.
        transaction_id: The transaction id that created the output.
        transaction_output_index: The output index.
    """
    type: int
    transaction_id: HexStr
    transaction_output_index: int


@json
@dataclass
class TreasuryInput:
    """A treasury input.

    Attributes:
        type: The type of input.
        milestone_id: The milestone id that created the treasury output.
    """
    type: int
    milestone_id: HexStr
