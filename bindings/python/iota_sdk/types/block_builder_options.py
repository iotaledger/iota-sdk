# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from typing import Any, Dict, List, Optional
from dataclasses import dataclass
from iota_sdk.types.common import HexStr, AddressAndAmount, json
from iota_sdk.client._high_level_api import Range
from iota_sdk.types.burn import Burn
from iota_sdk.types.output import Output
from iota_sdk.types.input import UtxoInput


@json
@dataclass
class BlockBuilderOptions:
    """Options to build a block.
    """
    total: str
    available: str
    coin_type: Optional[int] = None
    account_index: Optional[int] = None
    initial_address_index: Optional[int] = None
    inputs: Optional[List[UtxoInput]] = None
    input_range: Optional[Range] = None
    output: Optional[AddressAndAmount] = None
    output_hex: Optional[List[Dict[str, Any]]] = None
    outputs: Optional[List[Output]] = None
    custom_remainder_address: Optional[str] = None
    tag: Optional[HexStr] = None
    data: Optional[HexStr] = None
    parents: Optional[List[HexStr]] = None
    burn: Optional[Burn] = None
