# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from dataclasses import dataclass, field
from enum import IntEnum
from iota_sdk.types.signature import Ed25519Signature


class UnlockType(IntEnum):
    """Unlock variants.

    Attributes:
        Signature (0): An unlock holding a signature unlocking one or more inputs.
        Reference (1): An unlock which must reference a previous unlock which unlocks also the input at the same index as this Reference Unlock.
        Alias (2): An unlock which must reference a previous unlock which unlocks the alias that the input is locked to.
        Nft (3): An unlock which must reference a previous unlock which unlocks the NFT that the input is locked to.
    """
    Signature = 0
    Reference = 1
    Alias = 2
    Nft = 3


@dataclass
class Unlock:
    """Unlock type.
    """
    type: int

    def as_dict(self):
        """Converts this object to a dict.
        """
        config = {k: v for k, v in self.__dict__.items() if v is not None}

        if 'signature' in config:
            config['signature'] = config['signature'].__dict__

        return config


@dataclass
class SignatureUnlock(Unlock):
    """An unlock holding a signature unlocking one or more inputs.
    """
    signature: Ed25519Signature
    type: int = field(
        default_factory=lambda: int(
            UnlockType.Signature),
        init=False)


@dataclass
class ReferenceUnlock(Unlock):
    """An unlock which must reference a previous unlock which unlocks also the input at the same index as this Reference Unlock.
    """
    reference: int
    type: int = field(
        default_factory=lambda: int(
            UnlockType.Reference),
        init=False)


@dataclass
class AliasUnlock:
    """An unlock which must reference a previous unlock which unlocks the alias that the input is locked to.
    """
    reference: int
    type: int = field(
        default_factory=lambda: int(
            UnlockType.Alias),
        init=False)


@dataclass
class NftUnlock:
    """An unlock which must reference a previous unlock which unlocks the NFT that the input is locked to.
    """
    reference: int
    type: int = field(default_factory=lambda: int(UnlockType.Nft), init=False)
