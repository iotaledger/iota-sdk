# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from enum import IntEnum
from dataclasses import dataclass
from typing import TypeAlias, Union
from iota_sdk.types.common import json
from iota_sdk.types.block.basic import BasicBlock
from iota_sdk.types.block.validation import ValidationBlock


class BlockType(IntEnum):
    """Block types.

    Attributes:
        Basic (0): A Basic Block.
        Validation (1): A Validation Block.
    """
    Basic = 0
    Validation = 1


Block: TypeAlias = Union[BasicBlock, ValidationBlock]
