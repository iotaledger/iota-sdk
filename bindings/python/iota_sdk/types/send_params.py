# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, List
from iota_sdk.types.common import HexStr
from iota_sdk.types.native_token import NativeToken


@dataclass
class SendParams():
    """Parameters for sending base coins.

    Attributes:
        address: The address to send to.
        amount: The amount to send.
        returnAddress: The address to return the funds to if not claimed.
        expiration: Expiration in seconds, after which the output will be available for the sender again, if not spent by the
        receiver already. The expiration will only be used if one is necessary given the provided amount. If an
        expiration is needed but not provided, it will default to one day.
    """
    address: str
    amount: str
    returnAddress: Optional[str] = None
    expiration: Optional[int] = None

    def as_dict(self):
        """Converts this object to a dict.
        """
        config = {k: v for k, v in self.__dict__.items() if v is not None}

        config['amount'] = str(config['amount'])

        return config


@dataclass
class SendNativeTokensParams():
    """Parameters for sending native tokens

    Attributes:
        address: The address to send to.
        nativeTokens: The native tokens to send.
        returnAddress: The address to return the native tokens to if not claimed.
        expiration: The expiration timestamp until native tokens can be claimed.
    """
    address: str
    nativeTokens: List[NativeToken]
    returnAddress: Optional[str] = None
    expiration: Optional[int] = None

    def as_dict(self):
        """Converts this object to a dict.
        """
        return {k: v for k, v in self.__dict__.items() if v is not None}


@dataclass
class SendNftParams():
    """Parameters for sending NFTs.

    Attributes:
        address: The address to send the NFT to.
        nftId: The ID of the NFT to send.
    """
    address: str
    nftId: HexStr


@dataclass
class CreateNativeTokenParams():
    """Parameters for creating native tokens.

    Attributes:
        circulatingSupply: The circulating supply of the native token.
        maximumSupply: The maximum supply of the native token.
        foundryMetadata: The foundry metadata of the native token.
        aliasId: The ID of the corresponding alias.
    """
    circulatingSupply: int
    maximumSupply: int
    foundryMetadata: Optional[str] = None
    aliasId: Optional[str] = None

    def as_dict(self):
        """Converts this object to a dict.
        """
        config = {k: v for k, v in self.__dict__.items() if v is not None}

        config['circulatingSupply'] = hex(config['circulatingSupply'])
        config['maximumSupply'] = hex(config['maximumSupply'])

        return config


@dataclass
class MintNftParams():
    """Parameters for minting NFTs.

    Attributes:
        address: A Bech32 encoded address to which the NFT will be minted. Default will use the first address of the account.
        sender: An NFT sender feature.
        metadata: An NFT metadata feature.
        tag: An NFT tag feature.
        issuer: An NFT issuer feature.
        immutableMetadata: An NFT immutable metadata feature.
    """
    address: Optional[str] = None
    sender: Optional[str] = None
    metadata: Optional[str] = None
    tag: Optional[str] = None
    issuer: Optional[str] = None
    immutableMetadata: Optional[str] = None


@dataclass
class CreateAliasOutputParams():
    """Parameters for creating aliases.

    Attributes:
        address: A Bech32 encoded address which will control the alias. Default will use the first address of the account.
        immutableMetadata: Immutable alias metadata.
        metadata: Alias metadata.
        stateMetadata: Alias state metadata.
    """
    address: str
    immutableMetadata: Optional[str] = None
    metadata: Optional[str] = None
    stateMetadata: Optional[str] = None
