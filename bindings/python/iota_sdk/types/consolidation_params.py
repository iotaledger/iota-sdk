# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from dataclasses import dataclass
from typing import Optional


@dataclass
class ConsolidationParams:
    """Parameters for consolidation.
    """

    force: bool
    outputThreshold: Optional[int] = None
    targetAddress: Optional[str] = None
