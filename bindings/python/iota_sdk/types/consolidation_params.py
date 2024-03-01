# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from typing import Optional
from dataclasses import dataclass
from iota_sdk.types.common import json


@json
@dataclass
class ConsolidationParams:
    """Parameters for consolidation.

        Attributes:
        force (bool):
            Ignores the output_threshold if set to `true`.
        output_threshold (Optional[int]):
            Consolidates if the output number is >= the output_threshold.
        target_address (Optional[str]):
            Address to which the consolidated output should be sent.
    """

    force: bool
    output_threshold: Optional[int] = None
    target_address: Optional[str] = None
