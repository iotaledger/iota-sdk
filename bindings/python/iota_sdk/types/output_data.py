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
        outputId (HexStr): the associated output ID
        metadata (OutputMetadata): metadata associated with the output
        output (Output): the output itself
        isSpent (bool): whether the output is spent
        address (Address): the address associated with the output
        networkId (str): the network ID associated with the output
        remainder (bool): whether the output is a remainder
        chain (List[int], optional): array of chain state indexes 
    """

    outputId: HexStr
    metadata: OutputMetadata
    output: Output
    isSpent: bool
    address: Address
    networkId: str
    remainder: bool
    chain: Optional[Bip44] = None
