# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from dataclasses import dataclass
from typing import List, Optional
from iota_sdk.types.common import HexStr


@dataclass
class BaseCoinBalance:
    """Base coin fields for Balance.

    Attributes:
        total (str): The total balance.
        available (str): The available amount of the total balance.
    """
    total: str
    available: str


@dataclass
class RequiredStorageDeposit:
    """Required storage deposit for the outputs in the account.

    Attributes:
        alias (str): required amount for alias outputs.
        basic (str): required amount for basic outputs.
        foundry (str): required amount for foundry outputs.
        nft (str): required amount for nft outputs.
    """
    alias: str
    basic: str
    foundry: str
    nft: str


@dataclass
class NativeTokensBalance:
    """Native tokens fields for Balance.

    Attributes:
        tokenId (HexStr): The native token id.
        total (HexStr): The total native token balance.
        available (HexStr): The available amount of the total native token balance.
        metadata (HexStr): Some optional metadata of the native token.
    """
    tokenId: HexStr
    total: HexStr
    available: HexStr
    metadata: Optional[HexStr]


@dataclass
class Balance:
    """The balance of an account.

    Attributes:
        baseCoin (BaseCoinBalance): The base coin balance.
        requiredStorageDeposit (RequiredStorageDeposit): The required storage deposit.
        nativeTokens (List[NativeTokensBalance]): The balances of all native tokens.
        nfts (List[HexStr]): The owned NFTs.
        aliases (List[HexStr]): The owned aliases.
        foundries (List[HexStr]): The owned foundries.
        potentiallyLockedOutputs (dict[HexStr, bool]): A list of potentially locked outputs.
    """
    baseCoin: BaseCoinBalance
    requiredStorageDeposit: RequiredStorageDeposit
    nativeTokens: List[NativeTokensBalance]
    nfts: List[HexStr]
    aliases: List[HexStr]
    foundries: List[HexStr]
    potentiallyLockedOutputs: dict[HexStr, bool]

    def as_dict(self):
        config = {k: v for k, v in self.__dict__.items()}

        config['baseCoin'] = config['baseCoin'].__dict__
        config['requiredStorageDeposit'] = config['requiredStorageDeposit'].__dict__
        config['nativeTokens'] = [nt.__dict__
                                  for nt in config['nativeTokens']]

        return config
