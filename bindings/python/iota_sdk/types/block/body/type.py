# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from enum import IntEnum


class BlockBodyType(IntEnum):
    """Block Body types.

    Attributes:
        Basic (0): A Basic Block Body.
        Validation (1): A Validation Block Body.
    """
    Basic = 0
    Validation = 1
