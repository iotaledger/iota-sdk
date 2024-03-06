# Copyright 2024 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from dataclasses import dataclass, field
from dataclasses_json import config
from iota_sdk.types.common import json


@json
@dataclass
class NetworkMetricsResponse:
    """Network metrics.

    Attributes:
        blocks_per_second: The current rate of new blocks per second.
        confirmed_blocks_per_second: The current rate of confirmed blocks per second.
        confirmation_rate: The ratio of confirmed blocks to new blocks of the last confirmed slot.
    """
    blocks_per_second: float = field(metadata=config(
        encoder=str
    ))
    confirmed_blocks_per_second: float = field(metadata=config(
        encoder=str
    ))
    confirmation_rate: float = field(metadata=config(
        encoder=str
    ))
