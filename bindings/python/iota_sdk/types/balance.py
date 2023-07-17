# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from dataclasses import dataclass
from typing import List, Optional
from iota_sdk.types.common import HexStr


@dataclass
class BaseCoinBalance:
    """Base coin fields for Balance.
    """
    total: str
    available: str


@dataclass
class RequiredStorageDeposit:
    """Required storage deposit for the outputs in the account.
    """
    alias: str
    basic: str
    foundry: str
    nft: str


@dataclass
class NativeTokensBalance:
    """Native tokens fields for Balance.
    """
    tokenId: HexStr
    total: HexStr
    available: HexStr
    metadata: Optional[HexStr]


@dataclass
class Balance:
    """The balance of an account.
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
