# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from typing import Any, Dict, List, Optional
from dataclasses import dataclass, field
from dataclasses_json import config
from iota_sdk.types.common import HexStr, AddressAndAmount, json
from iota_sdk.client._high_level_api import Range
from iota_sdk.types.block.id import BlockId
from iota_sdk.types.burn import Burn
from iota_sdk.types.output import Output
from iota_sdk.types.input import UtxoInput


@json
@dataclass
class BlockBuilderOptions:
    """Options to build a block.
    """
    total: int = field(metadata=config(
        encoder=str
    ))
    available: int = field(metadata=config(
        encoder=str
    ))
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
    strong_parents: Optional[List[BlockId]] = None
    weak_parents: Optional[List[BlockId]] = None
    shallow_like_parents: Optional[List[BlockId]] = None
    burn: Optional[Burn] = None
