# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, List
from iota_sdk.types.common import HexStr
from iota_sdk.types.native_token import NativeToken


@dataclass
class SendParams():
    address: str
    amount: str
    returnAddress: Optional[str] = None
    expiration: Optional[int] = None

    def as_dict(self):
        config = {k: v for k, v in self.__dict__.items() if v != None}

        config['amount'] = str(config['amount'])

        return config


@dataclass
class SendNativeTokensParams():
    address: str
    nativeTokens: List[NativeToken]
    returnAddress: Optional[str] = None
    expiration: Optional[int] = None

    def as_dict(self):
        config = {k: v for k, v in self.__dict__.items() if v != None}

        config['amount'] = str(config['amount'])
        config['native_tokens'] = [native_token.as_dict()
                                   for native_token in config['native_tokens']]

        return config


@dataclass
class SendNftParams():
    address: str
    nftId: HexStr


@dataclass
class CreateNativeTokenParams():
    circulatingSupply: int
    maximumSupply: int
    foundryMetadata: Optional[str] = None
    aliasId: Optional[str] = None

    def as_dict(self):
        config = {k: v for k, v in self.__dict__.items() if v != None}

        config['circulatingSupply'] = hex(config['circulatingSupply'])
        config['maximumSupply'] = hex(config['maximumSupply'])

        return config


@dataclass
class MintNftParams():
    address: Optional[str] = None
    sender: Optional[str] = None
    metadata: Optional[str] = None
    tag: Optional[str] = None
    issuer: Optional[str] = None
    immutableMetadata: Optional[str] = None


@dataclass
class CreateAliasOutputParams():
    address: str
    immutableMetadata: Optional[str] = None
    metadata: Optional[str] = None
    stateMetadata: Optional[str] = None
