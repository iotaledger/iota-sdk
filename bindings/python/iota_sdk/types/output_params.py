# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from dataclasses import dataclass
from enum import Enum
from typing import List, Optional
from iota_sdk.types.common import HexStr, json
from iota_sdk.types.native_token import NativeToken


@json
@dataclass
class Assets():
    """Assets for OutputParams.
    """
    native_tokens: Optional[List[NativeToken]] = None
    nft_id: Optional[HexStr] = None


@json
@dataclass
class Features():
    """Features for OutputParams.
    """
    tag: Optional[HexStr] = None
    metadata: Optional[HexStr] = None
    issuer: Optional[str] = None
    sender: Optional[str] = None


@json
@dataclass
class Unlocks():
    """Unlocks for OutputParams.
    """
    expiration_unix_time: Optional[int] = None
    timelock_unix_time: Optional[int] = None


class ReturnStrategy(str, Enum):
    """Return strategy for the StorageDeposit.
    """
    Return = 'Return'
    Gift = 'Gift'


@json
@dataclass
class StorageDeposit():
    """Storage deposit options for OutputParams.
    """
    return_strategy: Optional[ReturnStrategy] = None
    use_excess_if_low: Optional[bool] = None


@json
@dataclass
class OutputParams():
    """Params for `Account.prepare_output()`.
    """
    recipient_address: str
    amount: str
    assets: Optional[Assets] = None
    features: Optional[Features] = None
    unlocks: Optional[Unlocks] = None
    storage_deposit: Optional[StorageDeposit] = None
