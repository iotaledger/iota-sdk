# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from dataclasses import dataclass
from typing import Any, Dict, List, Optional
from iota_sdk.types.common import HexStr, AddressAndAmount
from iota_sdk.client._high_level_api import Range
from iota_sdk.types.burn import Burn
from iota_sdk.types.output import Output
from iota_sdk.types.input import UtxoInput


@dataclass
class BlockBuilderOptions:
    """Options to build a block.
    """
    total: str
    available: str
    coinType: Optional[int] = None
    accountInde: Optional[int] = None
    initialAddressIndex: Optional[int] = None
    inputs: Optional[List[UtxoInput]] = None
    inputRange: Optional[Range] = None
    output: Optional[AddressAndAmount] = None
    outputHex: Optional[List[Dict[str, Any]]] = None
    outputs: Optional[List[Output]] = None
    customRemainderAddress: Optional[str] = None
    tag: Optional[HexStr] = None
    data: Optional[HexStr] = None
    parents: Optional[List[HexStr]] = None
    burn: Optional[Burn] = None
