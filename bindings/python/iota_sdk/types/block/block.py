# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from enum import IntEnum
from dataclasses import dataclass
from iota_sdk.types.common import json


class BlockType(IntEnum):
    """Block types.

    Attributes:
        Basic (0): A Basic Block.
        Validation (1): A Validation Block.
    """
    Basic = 0
    Validation = 1


@json
@dataclass
class Block:
    """Base class for blocks.
    """
    type: int
