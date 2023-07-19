# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from dataclasses import dataclass
from typing import Optional
from iota_sdk.types.address import Address
from iota_sdk.types.common import HexStr
from iota_sdk.types.output import Output, OutputMetadata
from iota_sdk.types.signature import Bip44


@dataclass
class OutputData():
    """Output data.

    Attributes:
        outputId: With the output data corresponding output ID.
        metadata: With the output corresponding metadata.
        output: The output object itself.
        isSpent: Whether the output is spent.
        address: The address associated with the output.
        networkId: The network ID the output belongs to.
        remainder: Whether the output represents a remainder amount.
        chain: A list of chain state indexes.
    """

    outputId: HexStr
    metadata: OutputMetadata
    output: Output
    isSpent: bool
    address: Address
    networkId: str
    remainder: bool
    chain: Optional[Bip44] = None
