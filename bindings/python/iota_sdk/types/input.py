# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from dataclasses import dataclass
from enum import IntEnum
from iota_sdk.types.common import HexStr


class InputType(IntEnum):
    """Input types.

    Attributes:
        Utxo: An unspent transaction output.
        Treasury: The treasury output.
    """
    Utxo = 0
    Treasury = 1


@dataclass
class UtxoInput:
    """Represents an input referencing an output.

    Attributes:
        type: The type of input.
        transactionId: The transaction id that created the output.
        transactionOutputIndex: The output index.
    """
    type: int
    transactionId: HexStr
    transactionOutputIndex: int


@dataclass
class TreasuryInput:
    """A treasury input.

    Attributes:
        type: The type of input.
        milestoneId: The milestone id that created the treasury output.
    """
    type: int
    milestoneId: HexStr
