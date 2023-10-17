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
        total: The total balance.
        available: The available amount of the total balance.
    """
    total: str
    available: str


@dataclass
class RequiredStorageDeposit:
    """Required storage deposit for the outputs in the account.

    Attributes:
        alias: The required amount for alias outputs.
        basic: The required amount for basic outputs.
        foundry: The required amount for foundry outputs.
        nft: The required amount for nft outputs.
    """
    alias: str
    basic: str
    foundry: str
    nft: str


@dataclass
class NativeTokensBalance:
    """Native tokens fields for Balance.

    Attributes:
        tokenId: The native token id.
        total: The total native token balance.
        available: The available amount of the total native token balance.
        metadata: Some metadata of the native token.
    """
    tokenId: HexStr
    total: HexStr
    available: HexStr
    metadata: Optional[HexStr]


@dataclass
class Balance:
    """The balance of an account.

    Attributes:
        baseCoin: The base coin balance.
        requiredStorageDeposit: The required storage deposit.
        nativeTokens: The balances of all native tokens.
        nfts: All owned NFTs.
        aliases: All owned aliases.
        foundries: All owned foundries.
        potentiallyLockedOutputs: A list of potentially locked outputs.
    """
    baseCoin: BaseCoinBalance
    requiredStorageDeposit: RequiredStorageDeposit
    nativeTokens: List[NativeTokensBalance]
    nfts: List[HexStr]
    aliases: List[HexStr]
    foundries: List[HexStr]
    potentiallyLockedOutputs: dict[HexStr, bool]

    def as_dict(self):
        """Converts this object to a dict.
        """
        config = self.__dict__

        config['baseCoin'] = config['baseCoin'].__dict__
        config['requiredStorageDeposit'] = config['requiredStorageDeposit'].__dict__
        config['nativeTokens'] = [nt.__dict__
                                  for nt in config['nativeTokens']]

        return config
