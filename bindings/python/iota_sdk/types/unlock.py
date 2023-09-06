# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from dataclasses import dataclass, field
from enum import IntEnum
from iota_sdk.types.signature import Ed25519Signature
from iota_sdk.types.common import json


class UnlockType(IntEnum):
    Signature = 0
    Reference = 1
    Alias = 2
    Nft = 3


@json
@dataclass
class Unlock:
    """Unlock type.
    """
    type: int


@json
@dataclass
class SignatureUnlock(Unlock):
    """An unlock holding a signature unlocking one or more inputs.
    """
    signature: Ed25519Signature
    type: int = field(
        default_factory=lambda: int(
            UnlockType.Signature),
        init=False)


@json
@dataclass
class ReferenceUnlock(Unlock):
    """An unlock which must reference a previous unlock which unlocks also the input at the same index as this Reference Unlock.
    """
    reference: int
    type: int = field(
        default_factory=lambda: int(
            UnlockType.Reference),
        init=False)


@json
@dataclass
class AliasUnlock:
    """An unlock which must reference a previous unlock which unlocks the alias that the input is locked to.
    """
    reference: int
    type: int = field(
        default_factory=lambda: int(
            UnlockType.Alias),
        init=False)


@json
@dataclass
class NftUnlock:
    """An unlock which must reference a previous unlock which unlocks the NFT that the input is locked to.
    """
    reference: int
    type: int = field(default_factory=lambda: int(UnlockType.Nft), init=False)
