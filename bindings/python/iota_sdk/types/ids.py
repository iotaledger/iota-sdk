# Copyright 2024 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from dataclasses import dataclass
from iota_sdk.types.common import json


@json
@dataclass
class IdWithSlotIndex(str):
    """Represents an hex encoded ID that contains a slot index at the end.

    Attributes:
        id: The hex encoded ID with a slot index.

    """
    id: str

    def slot_index(self):
        """Returns the slot index of the ID.
        """
        return int.from_bytes(
            bytes.fromhex(self.id[-8:]), 'little')


class BlockId(IdWithSlotIndex):
    """Represents a block ID.
    """


class TransactionId(IdWithSlotIndex):
    """Represents a transaction ID.
    """


class SlotCommitmentId(IdWithSlotIndex):
    """Represents a slot commitment ID.
    """
