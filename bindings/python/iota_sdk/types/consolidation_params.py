# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from dataclasses import dataclass
from typing import Optional


@dataclass
class ConsolidationParams:
    """Parameters for consolidation.

        Attributes:
        force (bool):
            Ignores the output_threshold if set to `true`.
        outputThreshold (Optional[int]):
            Consolidates if the output number is >= the output_threshold.
        targetAddress (Optional[str]):
            Address to which the consolidated output should be sent.
    """

    force: bool
    outputThreshold: Optional[int] = None
    targetAddress: Optional[str] = None
