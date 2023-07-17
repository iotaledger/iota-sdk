# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from dataclasses import dataclass
from enum import Enum
from typing import List, Optional
from iota_sdk.types.common import HexStr
from iota_sdk.types.native_token import NativeToken


@dataclass
class Assets():
    """Assets for OutputParams.
    """
    nativeTokens: Optional[List[NativeToken]] = None
    nftId: Optional[HexStr] = None


@dataclass
class Features():
    """Features for OutputParams.
    """
    tag: Optional[HexStr] = None
    metadata: Optional[HexStr] = None
    issuer: Optional[str] = None
    sender: Optional[str] = None


@dataclass
class Unlocks():
    """Unlocks for OutputParams.
    """
    expirationUnixTime: Optional[int] = None
    timelockUnixTime: Optional[int] = None


class ReturnStrategy(str, Enum):
    """Return strategy for the StorageDeposit.
    """
    Return = 'Return'
    Gift = 'Gift'


@dataclass
class StorageDeposit():
    """Storage deposit options for OutputParams.
    """
    returnStrategy: Optional[ReturnStrategy] = None
    useExcessIfLow: Optional[bool] = None


@dataclass
class OutputParams():
    """Params for `Account.prepare_output()`.
    """
    recipientAddress: str
    amount: str
    assets: Optional[Assets] = None
    features: Optional[Features] = None
    unlocks: Optional[Unlocks] = None
    storageDeposit: Optional[StorageDeposit] = None
