# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from dataclasses import dataclass
from typing import Optional
from iota_sdk.types.address import Address
from iota_sdk.types.common import HexStr, json
from iota_sdk.types.output import Output
from iota_sdk.types.output_metadata import OutputMetadata
from iota_sdk.types.signature import Bip44


@json
@dataclass
class OutputData():
    """Output data.

    Attributes:
        output_id: With the output data corresponding output ID.
        metadata: With the output corresponding metadata.
        output: The output object itself.
        address: The address associated with the output.
        network_id: The network ID the output belongs to.
        remainder: Whether the output represents a remainder amount.
        chain: A list of chain state indexes.
    """
    output_id: HexStr
    metadata: OutputMetadata
    output: Output
    address: Address
    network_id: str
    remainder: bool
    chain: Optional[Bip44] = None
