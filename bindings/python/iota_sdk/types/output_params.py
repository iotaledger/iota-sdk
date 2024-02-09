# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from dataclasses import dataclass, field
from enum import Enum
from typing import Optional
from dataclasses_json import config
from iota_sdk.types.common import HexStr, json, opt_int_encoder
from iota_sdk.types.native_token import NativeToken


@json
@dataclass
class Assets:
    """Assets for OutputParams.
    """
    nft_id: Optional[HexStr] = None


@json
@dataclass
class Features:
    """Features for OutputParams.
    """
    tag: Optional[HexStr] = None
    metadata: Optional[HexStr] = None
    issuer: Optional[str] = None
    sender: Optional[str] = None
    native_token: Optional[NativeToken] = None


@json
@dataclass
class Unlocks:
    """Unlocks for OutputParams.
    """
    expiration_slot_index: Optional[int] = field(default=None, metadata=config(
        encoder=opt_int_encoder
    ))
    timelock_slot_index: Optional[int] = field(default=None, metadata=config(
        encoder=opt_int_encoder
    ))


class ReturnStrategy(str, Enum):
    """Return strategy for the StorageDeposit.
    """
    Return = 'Return'
    Gift = 'Gift'


@json
@dataclass
class StorageDeposit:
    """Storage deposit options for OutputParams.
    """
    return_strategy: Optional[ReturnStrategy] = None
    use_excess_if_low: Optional[bool] = None


@json
@dataclass
class OutputParams:
    """Params for `Wallet.prepare_output()`.
    """
    recipient_address: str
    amount: int = field(metadata=config(
        encoder=str
    ))
    assets: Optional[Assets] = None
    features: Optional[Features] = None
    unlocks: Optional[Unlocks] = None
    storage_deposit: Optional[StorageDeposit] = None
