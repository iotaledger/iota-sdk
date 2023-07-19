# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from dataclasses import dataclass
from iota_sdk.types.common import HexStr, CoinType


@dataclass
class Ed25519Signature():
    """An Ed25519 signature.

    Attributes:
        publicKey: The Ed25519 public key.
        signature: The Ed25519 signature of some message.
        type: The Ed25519 signature type code.
    """
    publicKey: HexStr
    signature: HexStr
    type: int = 0


@dataclass
class Bip44():
    """TODO.
    """
    coinType: int = CoinType.IOTA
    account: int = 0
    change: int = 0
    addressIndex: int = 0
