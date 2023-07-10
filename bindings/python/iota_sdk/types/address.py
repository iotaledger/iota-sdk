# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from iota_sdk.types.common import HexStr
from dataclasses import dataclass
from enum import IntEnum
from typing import Optional


class AddressType(IntEnum):
    ED25519 = 0
    ALIAS = 8
    NFT = 16


@dataclass
class Address():
    type: int
    pubKeyHash: Optional[HexStr] = None
    aliasId: Optional[HexStr] = None
    nftId: Optional[HexStr] = None

    def as_dict(self):
        return {k: v for k, v in self.__dict__.items() if v != None}


class Ed25519Address(Address):
    def __init__(self, address: HexStr):
        """Initialize an Ed25519Address

        Parameters
        ----------
        address : string
            The hex encoded address to use.
        """
        super().__init__(AddressType.ED25519, pubKeyHash=address)


class AliasAddress(Address):
    def __init__(self, address_or_id: HexStr):
        """Initialize an AliasAddress

        Parameters
        ----------
        address_or_id : string
            The hex encoded address to use.
        """
        super().__init__(AddressType.ALIAS, aliasId=address_or_id)


class NFTAddress(Address):
    def __init__(self, address_or_id: HexStr):
        """Initialize an NFTokenAddress

        Parameters
        ----------
        address_or_id : string
            The hex encoded address to use.
        """
        super().__init__(AddressType.NFT, nftId=address_or_id)
