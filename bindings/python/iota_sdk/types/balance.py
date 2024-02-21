# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from typing import List, Optional
from dataclasses import dataclass, field
from dataclasses_json import config
from iota_sdk.types.decayed_mana import DecayedMana
from iota_sdk.types.common import hex_str_decoder, HexStr, json
from iota_sdk.types.output_id import OutputId


@json
@dataclass
class BaseCoinBalance:
    """Base coin fields for Balance.

    Attributes:
        total: The total balance.
        available: The available amount of the total balance.
        # voting_power: The voting power of the wallet.
    """
    total: int = field(metadata=config(
        encoder=str
    ))
    available: int = field(metadata=config(
        encoder=str
    ))
    # TODO https://github.com/iotaledger/iota-sdk/issues/1822
    # voting_power: int = field(metadata=config(
    # encoder=str
    # ))


@json
@dataclass
class ManaBalance:
    """Mana fields for Balance.

    Attributes:
        total: The total balance.
        available: The available amount of the total balance.
        rewards: Mana rewards of account and delegation outputs.
    """
    total: DecayedMana
    available: DecayedMana
    rewards: int = field(metadata=config(
        encoder=str
    ))


@json
@dataclass
class RequiredStorageDeposit:
    """Required storage deposit for the outputs in the wallet.

    Attributes:
        basic: The required amount for basic outputs.
        account: The required amount for account outputs.
        foundry: The required amount for foundry outputs.
        nft: The required amount for nft outputs.
        delegation: The required amount for delegation outputs.
    """
    basic: int = field(metadata=config(
        encoder=str
    ))
    account: int = field(metadata=config(
        encoder=str
    ))
    foundry: int = field(metadata=config(
        encoder=str
    ))
    nft: int = field(metadata=config(
        encoder=str
    ))
    delegation: int = field(metadata=config(
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
    total: int = field(metadata=config(
        encoder=hex,
        decoder=hex_str_decoder,
    ))
    available: int = field(metadata=config(
        encoder=hex,
        decoder=hex_str_decoder,
    ))
    metadata: Optional[HexStr]


@json
@dataclass
class Balance:
    """The balance of an account.

    Attributes:
        base_coin: The base coin balance.
        mana: Total and available mana.
        required_storage_deposit: The required storage deposit.
        native_tokens: The balances of all native tokens.
        accounts: All owned accounts.
        foundries: All owned foundries.
        nfts: All owned NFTs.
        delegations: All owned delegation outputs.
        potentially_locked_outputs: A list of potentially locked outputs.
    """
    base_coin: BaseCoinBalance
    mana: ManaBalance
    required_storage_deposit: RequiredStorageDeposit
    native_tokens: dict[HexStr, NativeTokensBalance]
    accounts: List[HexStr]
    foundries: List[HexStr]
    nfts: List[HexStr]
    delegations: List[HexStr]
    potentially_locked_outputs: dict[OutputId, bool]
