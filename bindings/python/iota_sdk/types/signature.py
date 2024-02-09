# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from typing import TypeAlias
from dataclasses import dataclass, field
from iota_sdk.types.common import HexStr, CoinType, json


@json
@dataclass
class Ed25519Signature:
    """An Ed25519 signature.

    Attributes:
        public_key: The Ed25519 public key.
        signature: The Ed25519 signature of some message.
        type: The Ed25519 signature type.
    """
    type: int = field(default=0, init=False)
    public_key: HexStr
    signature: HexStr


Signature: TypeAlias = Ed25519Signature


@json
@dataclass
class Bip44:
    """A BIP44 chain.

    Attributes:
        coin_type: The coin type segment.
        account: The account segment.
        change: The change segment.
        address_index: The address index segment.
    """
    coin_type: int = CoinType.IOTA
    account: int = 0
    change: int = 0
    address_index: int = 0
