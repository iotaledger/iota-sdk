# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from enum import IntEnum
from dataclasses import dataclass, field
from typing import Any, Dict, List, Optional, TypeAlias, Union
from iota_sdk.types.common import HexStr, json


class AddressType(IntEnum):
    """Address type variants.

     Attributes:
        ED25519 (0): Ed25519 address.
        ACCOUNT (8): Account address.
        NFT (16): Nft address.
        ANCHOR (24): Anchor address.
        IMPLICIT_ACCOUNT_CREATION (32): Implicit Account Creation address.
        MULTI (40): Multi address.
        RESTRICTED (48): Address with restricted capabilities.

    """
    ED25519 = 0
    ACCOUNT = 8
    NFT = 16
    ANCHOR = 24
    IMPLICIT_ACCOUNT_CREATION = 32
    MULTI = 40
    RESTRICTED = 48


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
    pub_key_hash: HexStr


@json
@dataclass
class WeightedAddress:
    """An address with an assigned weight.
    Attributes:
        address: The unlocked address.
        weight: The weight of the unlocked address.
    """
    address: Union[Ed25519Address, AccountAddress, NFTAddress, AnchorAddress]
    weight: int


@json
@dataclass
class MultiAddress:
    """An address that consists of addresses with weights and a threshold value.
    The Multi Address can be unlocked if the cumulative weight of all unlocked addresses is equal to or exceeds the
    threshold.
    Attributes:
        addresses: The weighted unlocked addresses.
        threshold: The threshold that needs to be reached by the unlocked addresses in order to unlock the multi address.
    """
    type: int = field(default_factory=lambda: int(
        AddressType.MULTI), init=False)
    addresses: List[WeightedAddress]
    threshold: int


@json
@dataclass
class RestrictedAddress:
    """Represents an address with restricted capabilities.
    Attributes:
        address: The inner restricted Address.
        allowed_capabilities: The allowed capabilities bitflags.
    """
    type: int = field(default_factory=lambda: int(
        AddressType.RESTRICTED), init=False)
    address: Union[Ed25519Address, AccountAddress, NFTAddress]
    allowed_capabilities: Optional[HexStr] = field(default=None, init=False)

    def with_allowed_capabilities(self, capabilities: bytes):
        """Sets the allowed capabilities from a byte array.
        Attributes:
            capabilities: The allowed capabilities bitflags.
        """
        if any(c != 0 for c in capabilities):
            self.allowed_capabilities = '0x' + capabilities.hex()
        else:
            self.allowed_capabilities = None


Address: TypeAlias = Union[Ed25519Address,
                           AccountAddress,
                           NFTAddress,
                           AnchorAddress,
                           ImplicitAccountCreationAddress,
                           MultiAddress,
                           RestrictedAddress]


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
    if address_type == AddressType.MULTI:
        return MultiAddress.from_dict(d)
    if address_type == AddressType.RESTRICTED:
        return RestrictedAddress.from_dict(d)
    raise Exception(f'invalid address type: {address_type}')


def deserialize_addresses(
        dicts: List[Dict[str, Any]]) -> List[Address]:
    """
    Takes a list of dictionaries as input and returns a list with specific instances of classes based on the value of the 'type' key in the dictionary.

    Arguments:
    * `dicts`: A list of dictionaries that are expected to have a key called 'type' which specifies the type of the returned value.
    """
    return list(map(deserialize_address, dicts))
