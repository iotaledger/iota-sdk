# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, List
from iota_sdk.types.address import Address
from iota_sdk.types.common import HexStr
from iota_sdk.types.output import Output, OutputMetadata


@dataclass
class OutputData():
    outputId: HexStr
    metadata: OutputMetadata
    output: Output
    isSpent: bool
    address: Address
    networkId: str
    remainder: bool
    chain: Optional[List[int]] = None
