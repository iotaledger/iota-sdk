# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from typing import List, Optional
from dataclasses import dataclass, field
from dataclasses_json import config
from iota_sdk.types.common import HexStr, json


@json
@dataclass
class BaseCoinBalance:
    """Base coin fields for Balance.

    Attributes:
        total: The total balance.
        available: The available amount of the total balance.
    """
    total: int = field(metadata=config(
        encoder=str
    ))
    available: int = field(metadata=config(
        encoder=str
    ))


@json
@dataclass
class RequiredStorageDeposit:
    """Required storage deposit for the outputs in the account.

    Attributes:
        account: The required amount for account outputs.
        basic: The required amount for basic outputs.
        foundry: The required amount for foundry outputs.
        nft: The required amount for nft outputs.
    """
    account: int = field(metadata=config(
        encoder=str
    ))
    basic: int = field(metadata=config(
        encoder=str
    ))
    foundry: int = field(metadata=config(
        encoder=str
    ))
    nft: int = field(metadata=config(
        encoder=str
    ))


@json
@dataclass
class NativeTokensBalance:
    """Native tokens fields for Balance.

    Attributes:
        token_id: The native token id.
        total: The total native token balance.
        available: The available amount of the total native token balance.
        metadata: Some metadata of the native token.
    """
    token_id: HexStr
    total: int = field(metadata=config(
        encoder=str
    ))
    available: int = field(metadata=config(
        encoder=str
    ))
    metadata: Optional[HexStr]


@json
@dataclass
class Balance:
    """The balance of an account.

    Attributes:
        base_coin: The base coin balance.
        required_storage_deposit: The required storage deposit.
        native_tokens: The balances of all native tokens.
        nfts: All owned NFTs.
        accounts: All owned accounts.
        foundries: All owned foundries.
        potentially_locked_outputs: A list of potentially locked outputs.
    """
    base_coin: BaseCoinBalance
    required_storage_deposit: RequiredStorageDeposit
    native_tokens: List[NativeTokensBalance]
    nfts: List[HexStr]
    accounts: List[HexStr]
    foundries: List[HexStr]
    potentially_locked_outputs: dict[HexStr, bool]
