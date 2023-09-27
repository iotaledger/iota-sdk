# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from enum import IntEnum

from dataclasses import dataclass, field
from typing import Any, Dict, List, Union


from iota_sdk.types.common import HexStr, json


class AddressType(IntEnum):
    """Address type variants.

     Attributes:
        ED25519 (0): Ed25519 address.
        ACCOUNT (8): Account address.
        NFT (16): Nft address.
    """
    ED25519 = 0
    ACCOUNT = 8
    NFT = 16


@json
@dataclass
class Address():
    """Base class for addresses.
    """
    type: int


@json
@dataclass
class Ed25519Address(Address):
    """Represents an Ed25519 address.
    Attributes:
        pub_key_hash: The hex encoded Ed25519 public key hash.
    """
    pub_key_hash: HexStr
    type: int = field(
        default_factory=lambda: int(
            AddressType.ED25519),
        init=False)


@json
@dataclass
class AccountAddress(Address):
    """Represents an Account address.
    Attributes:
        account_id: The hex encoded account id.
    """
    account_id: HexStr
    type: int = field(
        default_factory=lambda: int(
            AddressType.ACCOUNT),
        init=False)


@json
@dataclass
class NFTAddress(Address):
    """Represents an NFT address.
    Attributes:
        nft_id: The hex encoded NFT id.
    """
    nft_id: HexStr
    type: int = field(default_factory=lambda: int(AddressType.NFT), init=False)


@json
@dataclass
class AddressWithUnspentOutputs():
    """An Address with unspent outputs.
    """
    address: str
    key_index: int
    internal: bool
    output_ids: bool


def address_from_dict(dict: Dict[str, Any]) -> Union[Ed25519Address, AccountAddress, NFTAddress]:
    """
    Takes a dictionary as input and returns an instance of a specific class based on the value of the 'type' key in the dictionary.

    Arguments:
    * `dict`: A dictionary that is expected to have a key called 'type' which specifies the type of the returned value.
    """
    type = dict['type']
    if type == AddressType.ED25519:
        return Ed25519Address.from_dict(dict)
    if type == AddressType.ACCOUNT:
        return AccountAddress.from_dict(dict)
    if type == AddressType.NFT:
        return NFTAddress.from_dict(dict)
    raise Exception(f'invalid address type: {type}')


def addresses_from_dicts(
        dicts: List[Dict[str, Any]]) -> List[Union[Ed25519Address, AccountAddress, NFTAddress]]:
    """
    Takes a list of dictionaries as input and returns a list with specific instances of a classes based on the value of the 'type' key in the dictionary.

    Arguments:
    * `dicts`: A list of dictionaries that are expected to have a key called 'type' which specifies the type of the returned value.
    """
    return list(map(address_from_dict, dicts))
