# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from enum import IntEnum
from dataclasses import dataclass, field
from typing import Any, Dict, List, TypeAlias, Union
from iota_sdk.types.common import HexStr, json


class AddressType(IntEnum):
    """Address type variants.

     Attributes:
        ED25519 (0): Ed25519 address.
        ACCOUNT (8): Account address.
        NFT (16): Nft address.
        ANCHOR (24): Anchor address.
        IMPLICIT_ACCOUNT_CREATION (32): Implicit Account Creation address.

    """
    ED25519 = 0
    ACCOUNT = 8
    NFT = 16
    ANCHOR = 24
    IMPLICIT_ACCOUNT_CREATION = 32


@json
@dataclass
class Ed25519Address:
    """Represents an Ed25519 address.
    Attributes:
        pub_key_hash: The hex encoded Ed25519 public key hash.
    """
    type: int = field(
        default_factory=lambda: int(
            AddressType.ED25519),
        init=False)
    pub_key_hash: HexStr


@json
@dataclass
class AccountAddress:
    """Represents an Account address.
    Attributes:
        account_id: The hex encoded account id.
    """
    type: int = field(
        default_factory=lambda: int(
            AddressType.ACCOUNT),
        init=False)
    account_id: HexStr


@json
@dataclass
class NFTAddress:
    """Represents an NFT address.
    Attributes:
        nft_id: The hex encoded NFT id.
    """
    type: int = field(default_factory=lambda: int(AddressType.NFT), init=False)
    nft_id: HexStr


@json
@dataclass
class AnchorAddress:
    """Represents an Anchor address.
    Attributes:
        anchor_id: The hex encoded anchor id.
    """
    type: int = field(
        default_factory=lambda: int(
            AddressType.ANCHOR),
        init=False)
    anchor_id: HexStr


@json
@dataclass
class ImplicitAccountCreationAddress:
    """An implicit account creation address that can be used to convert a Basic Output to an Account Output.
    Attributes:
        address: The hex encoded Ed25519 Address.
    """
    type: int = field(default_factory=lambda: int(
        AddressType.IMPLICIT_ACCOUNT_CREATION), init=False)
    address: Ed25519Address

    def to_dict(self) -> dict:
        """Converts an implicit account creation address to the dictionary representation.
        """

        return {
            "type": self.type,
            "pubKeyHash": self.address.pub_key_hash
        }

    @staticmethod
    def from_dict(addr_dict: dict):
        """Creates an implicit account creation address from a dictionary representation.
        """

        return ImplicitAccountCreationAddress(
            Ed25519Address(addr_dict['pubKeyHash']))


Address: TypeAlias = Union[Ed25519Address,
                           AccountAddress,
                           NFTAddress,
                           AnchorAddress,
                           ImplicitAccountCreationAddress]


# pylint: disable=too-many-return-statements
def deserialize_address(d: Dict[str, Any]) -> Address:
    """
    Takes a dictionary as input and returns an instance of a specific class based on the value of the 'type' key in the dictionary.

    Arguments:
    * `d`: A dictionary that is expected to have a key called 'type' which specifies the type of the returned value.
    """
    address_type = d['type']
    if address_type == AddressType.ED25519:
        return Ed25519Address.from_dict(d)
    if address_type == AddressType.ACCOUNT:
        return AccountAddress.from_dict(d)
    if address_type == AddressType.NFT:
        return NFTAddress.from_dict(d)
    if address_type == AddressType.ANCHOR:
        return AnchorAddress.from_dict(d)
    if address_type == AddressType.IMPLICIT_ACCOUNT_CREATION:
        return ImplicitAccountCreationAddress.from_dict(d)
    raise Exception(f'invalid address type: {address_type}')


def deserialize_addresses(
        dicts: List[Dict[str, Any]]) -> List[Address]:
    """
    Takes a list of dictionaries as input and returns a list with specific instances of classes based on the value of the 'type' key in the dictionary.

    Arguments:
    * `dicts`: A list of dictionaries that are expected to have a key called 'type' which specifies the type of the returned value.
    """
    return list(map(deserialize_address, dicts))
