# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from dataclasses import dataclass
from typing import Optional, List
from iota_sdk.types.common import HexStr, json
from iota_sdk.types.native_token import NativeToken


@json
@dataclass
class SendParams():
    """Parameters for sending base coins.

    Attributes:
        address: The address to send to.
        amount: The amount to send.
        return_address : The address to return the funds to if not claimed.
        expiration: The expiration timestamp until funds can be claimed.
    """
    address: str
    amount: str
    return_address: Optional[str] = None
    expiration: Optional[int] = None


@json
@dataclass
class SendNativeTokensParams():
    """Parameters for sending native tokens

    Attributes:
        address: The address to send to.
        native_tokens: The native tokens to send.
        return_address: The address to return the native tokens to if not claimed.
        expiration: The expiration timestamp until native tokens can be claimed.
    """
    address: str
    native_tokens: List[NativeToken]
    return_address: Optional[str] = None
    expiration: Optional[int] = None


@json
@dataclass
class SendNftParams():
    """Parameters for sending NFTs.

    Attributes:
        address: The address to send the NFT to.
        nft_id: The ID of the NFT to send.
    """
    address: str
    nft_id: HexStr


@json
@dataclass
class CreateNativeTokenParams():
    """Parameters for creating native tokens.

    Attributes:
        circulating_supply: The circulating supply of the native token.
        maximum_supply: The maximum supply of the native token.
        foundry_metadata: The foundry metadata of the native token.
        alias_id: The ID of the corresponding alias.
    """
    circulating_supply: int
    maximum_supply: int
    foundry_metadata: Optional[str] = None
    alias_id: Optional[str] = None

    @staticmethod
    def _to_dict_custom(config):
        config['circulatingSupply'] = hex(config['circulatingSupply'])
        config['maximumSupply'] = hex(config['maximumSupply'])

        return config


@json
@dataclass
class MintNftParams():
    """Parameters for minting NFTs.

    Attributes:
        address: A Bech32 encoded address to which the NFT will be minted. Default will use the first address of the account.
        sender: An NFT sender feature.
        metadata: An NFT metadata feature.
        tag: An NFT tag feature.
        issuer: An NFT issuer feature.
        immutable_metadata: An NFT immutable metadata feature.
    """
    address: Optional[str] = None
    sender: Optional[str] = None
    metadata: Optional[str] = None
    tag: Optional[str] = None
    issuer: Optional[str] = None
    immutable_metadata: Optional[str] = None


@json
@dataclass
class CreateAliasOutputParams():
    """Parameters for creating aliases.

    Attributes:
        address: A Bech32 encoded address which will control the alias. Default will use the first address of the account.
        immutable_metadata: Immutable alias metadata.
        metadata: Alias metadata.
        state_metadata: Alias state metadata.
    """
    address: str
    immutable_metadata: Optional[str] = None
    metadata: Optional[str] = None
    state_metadata: Optional[str] = None
