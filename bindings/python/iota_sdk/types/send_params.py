# Copyright 2024 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from dataclasses import dataclass, field
from typing import Optional
from dataclasses_json import config
from iota_sdk.types.address import AccountAddress
from iota_sdk.types.common import hex_str_decoder, HexStr, json
from iota_sdk.types.feature import MetadataFeature
from iota_sdk.types.native_token import NativeToken
from iota_sdk.types.output_params import ReturnStrategy


@json
@dataclass
class SendParams:
    """Parameters for sending base coins.

    Attributes:
        address: The address to send to.
        amount: The amount to send.
        return_address: The address to return the funds to if not claimed.
        expiration: Expiration in seconds, after which the output will be available for the sender again, if not spent by the
        receiver already. The expiration will only be used if one is necessary given the provided amount. If an
        expiration is needed but not provided, it will default to one day.
    """
    address: str
    amount: int = field(metadata=config(
        encoder=str
    ))
    return_address: Optional[str] = None
    expiration: Optional[int] = None


@json
@dataclass
class SendNativeTokenParams:
    """Parameters for sending a native token

    Attributes:
        address: The address to send to.
        native_token: The native token to send.
        return_address: The address to return the native token to if not claimed.
        expiration: The expiration timestamp until the native token can be claimed.
    """
    address: str
    native_token: NativeToken
    return_address: Optional[str] = None
    expiration: Optional[int] = None


@json
@dataclass
class SendNftParams:
    """Parameters for sending NFTs.

    Attributes:
        address: The address to send the NFT to.
        nft_id: The ID of the NFT to send.
    """
    address: str
    nft_id: HexStr


@json
@dataclass
class CreateNativeTokenParams:
    """Parameters for creating native tokens.

    Attributes:
        circulating_supply: The circulating supply of the native token.
        maximum_supply: The maximum supply of the native token.
        foundry_metadata: The foundry metadata of the native token.
        account_id: The ID of the corresponding account.
    """
    circulating_supply: int = field(metadata=config(
        encoder=hex,
        decoder=hex_str_decoder,
    ))
    maximum_supply: int = field(metadata=config(
        encoder=hex,
        decoder=hex_str_decoder,
    ))
    foundry_metadata: Optional[MetadataFeature] = None
    account_id: Optional[str] = None


@json
@dataclass
class CreateDelegationParams:
    """Parameters for creating a delegation output.

    Attributes:
        delegated_amount: The amount to delegate.
        validator_address: The Account Address of the validator to which this output will delegate.
        address: Bech32 encoded address which will control the delegation. By default, the ed25519 wallet address will be used.
    """
    delegated_amount: int = field(metadata=config(
        encoder=str
    ))
    validator_address: AccountAddress
    address: Optional[str] = None


@json
@dataclass
class BeginStakingParams:
    """Parameters for beginning staking.

    Attributes:
        account_id: The account id which will become a validator.
        staked_amount: The amount of tokens to stake.
        fixed_cost: The fixed cost of the validator, which it receives as part of its Mana rewards.
        staking_period: The staking period (in epochs). Will default to the staking unbonding period.
    """
    account_id: HexStr
    staked_amount: int
    fixed_cost: int
    staking_period: Optional[int] = None


@json
@dataclass
class MintNftParams:
    """Parameters for minting NFTs.

    Attributes:
        address: A Bech32 encoded address to which the NFT will be minted. Default will use the address of the wallet.
        sender: An NFT sender feature.
        metadata: An NFT metadata feature.
        tag: An NFT tag feature.
        issuer: An NFT issuer feature.
        immutable_metadata: An NFT immutable metadata feature.
    """
    address: Optional[str] = None
    sender: Optional[str] = None
    metadata: Optional[MetadataFeature] = None
    tag: Optional[str] = None
    issuer: Optional[str] = None
    immutable_metadata: Optional[MetadataFeature] = None


@json
@dataclass
class CreateAccountOutputParams:
    """Parameters for creating accounts.

    Attributes:
        address: A Bech32 encoded address which will control the account. Default will use the address of the wallet.
        immutable_metadata: Immutable account metadata.
        metadata: Account metadata.
    """
    address: str
    immutable_metadata: Optional[MetadataFeature] = None
    metadata: Optional[MetadataFeature] = None


@json
@dataclass
class SendManaParams:
    """Parameters for sending mana.

    Attributes:
        mana: Amount of mana to send, e.g. 1000000.
        address: Recipient address, e.g. rms1qztwng6cty8cfm42nzvq099ev7udhrnk0rw8jt8vttf9kpqnxhpsx869vr3.
        return_strategy: Whether to gift the storage deposit or not.
    """
    mana: int = field(metadata=config(
        encoder=str
    ))
    address: str
    return_strategy: Optional[ReturnStrategy] = None
