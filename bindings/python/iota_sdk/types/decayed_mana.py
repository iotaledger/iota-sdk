# Copyright 2024 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from dataclasses import dataclass
from iota_sdk.types.common import json


@json
@dataclass
class DecayedMana:
    """Decayed stored and potential Mana of an output.

        Attributes:
        stored (int):
            Decayed stored mana.
        potential (int):
            Decayed potential mana.
    """

    stored: int
    potential: int
