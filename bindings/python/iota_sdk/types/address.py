# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from enum import IntEnum

from dataclasses import dataclass, field


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
# pylint: disable=function-redefined
# TODO: Change name
class AccountAddress():
    """An Address of the Account.
    """
    address: str
    key_index: int
    internal: bool
    used: bool


@json
@dataclass
class AddressWithUnspentOutputs():
    """An Address with unspent outputs.
    """
    address: str
    key_index: int
    internal: bool
    output_ids: bool
