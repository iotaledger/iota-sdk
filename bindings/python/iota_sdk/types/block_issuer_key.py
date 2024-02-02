# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from enum import IntEnum
from typing import Any, Dict, List, TypeAlias
from dataclasses import dataclass, field
from iota_sdk.types.common import HexStr, json


class BlockIssuerKeyType(IntEnum):
    """BlockIssuerKey type variants.

     Attributes:
        Ed25519PublicKeyHash (0): Ed25519 public key hash block issuer key.
    """
    Ed25519PublicKeyHash = 0


@json
@dataclass
class Ed25519PublicKeyHashBlockIssuerKey:
    """A Block Issuer Key backed by an Ed25519 Public Key Hash.
    Attributes:
        pub_key_hash: The hex encoded Ed25519 public key hash.
    """
    type: int = field(
        default_factory=lambda: int(
            BlockIssuerKeyType.Ed25519PublicKeyHash),
        init=False)
    pub_key_hash: HexStr


BlockIssuerKey: TypeAlias = Ed25519PublicKeyHashBlockIssuerKey


def deserialize_block_issuer_key(d: Dict[str, Any]) -> BlockIssuerKey:
    """
    Takes a dictionary as input and returns an instance of a specific class based on the value of the 'type' key in the dictionary.

    Arguments:
    * `d`: A dictionary that is expected to have a key called 'type' which specifies the type of the returned value.
    """
    block_issuer_key_type = d['type']
    if block_issuer_key_type == block_issuer_key_type.Ed25519PublicKeyHash:
        return Ed25519PublicKeyHashBlockIssuerKey.from_dict(d)
    raise Exception(f'invalid block issuer key type: {block_issuer_key_type}')


def deserialize_block_issuer_keys(
        dicts: List[Dict[str, Any]]) -> List[BlockIssuerKey]:
    """
    Takes a list of dictionaries as input and returns a list with specific instances of classes based on the value of the 'type' key in the dictionary.

    Arguments:
    * `dicts`: A list of dictionaries that are expected to have a key called 'type' which specifies the type of the returned value.
    """
    return list(map(deserialize_block_issuer_key, dicts))
