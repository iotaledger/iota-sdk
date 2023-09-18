# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, Union
from iota_sdk.types.address import Ed25519Address, AccountAddress, NFTAddress
from iota_sdk.types.common import HexStr, json
from iota_sdk.types.output import BasicOutput, AccountOutput, FoundryOutput, NftOutput, OutputMetadata
from iota_sdk.types.signature import Bip44


@json
@dataclass
class OutputData():
    """Output data.

    Attributes:
        output_id: With the output data corresponding output ID.
        metadata: With the output corresponding metadata.
        output: The output object itself.
        is_spent: Whether the output is spent.
        address: The address associated with the output.
        network_id: The network ID the output belongs to.
        remainder: Whether the output represents a remainder amount.
        chain: A list of chain state indexes.
    """
    output_id: HexStr
    metadata: OutputMetadata
    output: Union[AccountOutput, FoundryOutput, NftOutput, BasicOutput]
    is_spent: bool
    address: Union[Ed25519Address, AccountAddress, NFTAddress]
    network_id: str
    remainder: bool
    chain: Optional[Bip44] = None
