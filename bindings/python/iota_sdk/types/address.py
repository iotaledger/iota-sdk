# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from dataclasses import dataclass, field
from enum import IntEnum
from iota_sdk.types.common import HexStr


class AddressType(IntEnum):
    """Address type variants.

     Attributes:
        ED25519 (0): Ed25519 address.
        ALIAS (8): Alias address.
        NFT (16): Nft address.
    """
    ED25519 = 0
    ALIAS = 8
    NFT = 16


@dataclass
class Address():
    """Base class for addresses.
    """
    type: int

    def as_dict(self):
        """Converts this object to a dict.
        """
        return {k: v for k, v in self.__dict__.items() if v is not None}


@dataclass
class Ed25519Address(Address):
    """Represents an Ed25519 address.
    Attributes:
        pubKeyHash: The hex encoded Ed25519 public key hash.
    """
    pubKeyHash: HexStr
    type: int = field(
        default_factory=lambda: int(
            AddressType.ED25519),
        init=False)


@dataclass
class AliasAddress(Address):
    """Represents an Alias address.
    Attributes:
        aliasId: The hex encoded alias id.
    """
    aliasId: HexStr
    type: int = field(
        default_factory=lambda: int(
            AddressType.ALIAS),
        init=False)


@dataclass
class NFTAddress(Address):
    """Represents an NFT address.
    Attributes:
        nftId: The hex encoded NFT id.
    """
    nftId: HexStr
    type: int = field(default_factory=lambda: int(AddressType.NFT), init=False)


@dataclass
class AccountAddress():
    """An Address of the Account.
    """
    address: str
    keyIndex: int
    internal: bool
    used: bool


@dataclass
class AddressWithUnspentOutputs():
    """An Address with unspent outputs.
    """
    address: str
    keyIndex: int
    internal: bool
    outputIds: bool
