# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from iota_sdk.types.common import HexStr
from dataclasses import dataclass
from enum import IntEnum
from typing import Optional


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

     Attributes:
        type: The address type.
        pubKeyHash: The hex encoded public key hash.
        aliasId: The hex encoded alias id.
        nftId: The hex encoded nft id.
    """
    type: int
    pubKeyHash: Optional[HexStr] = None
    aliasId: Optional[HexStr] = None
    nftId: Optional[HexStr] = None

    def as_dict(self):
        return {k: v for k, v in self.__dict__.items() if v is not None}


class Ed25519Address(Address):
    """Represents an Ed25519 address.
    """

    def __init__(self, address: HexStr):
        """Initialize an Ed25519Address

        Args:
            address: The hex encoded address to use.
        """
        super().__init__(AddressType.ED25519, pubKeyHash=address)


class AliasAddress(Address):
    """Represents an Alias address.
    """

    def __init__(self, address_or_id: HexStr):
        """Initialize an AliasAddress

        Args:
            address_or_id: The hex encoded address to use.
        """
        super().__init__(AddressType.ALIAS, aliasId=address_or_id)


class NFTAddress(Address):
    """Represents an NFT address.
    """

    def __init__(self, address_or_id: HexStr):
        """Initialize an NFTAddress

        Args:
            address_or_id: The hex encoded address to use.
        """
        super().__init__(AddressType.NFT, nftId=address_or_id)


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
