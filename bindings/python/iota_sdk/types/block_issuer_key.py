# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from enum import IntEnum
from dataclasses import dataclass, field
from typing import Any, Dict, List, TypeAlias
from iota_sdk.types.common import HexStr, json


class BlockIssuerKeyType(IntEnum):
    """BlockIssuerKey type variants.

     Attributes:
        ED25519 (0): Ed25519 block issuer key.
    """
    ED25519 = 0


@json
@dataclass
class Ed25519BlockIssuerKey:
    """A Block Issuer Key backed by an Ed25519 Public Key.
    Attributes:
        public_key: The hex encoded Ed25519 public key.
    """
    public_key: HexStr
    type: int = field(
        default_factory=lambda: int(
            BlockIssuerKeyType.ED25519),
        init=False)


BlockIssuerKey: TypeAlias = Ed25519BlockIssuerKey


def deserialize_block_issuer_key(d: Dict[str, Any]) -> BlockIssuerKey:
    """
    Takes a dictionary as input and returns an instance of a specific class based on the value of the 'type' key in the dictionary.

    Arguments:
    * `d`: A dictionary that is expected to have a key called 'type' which specifies the type of the returned value.
    """
    block_issuer_key_type = d['type']
    if block_issuer_key_type == block_issuer_key_type.ED25519:
        return Ed25519BlockIssuerKey.from_dict(d)
    raise Exception(f'invalid block issuer key type: {block_issuer_key_type}')


def deserialize_block_issuer_keys(
        dicts: List[Dict[str, Any]]) -> List[BlockIssuerKey]:
    """
    Takes a list of dictionaries as input and returns a list with specific instances of classes based on the value of the 'type' key in the dictionary.

    Arguments:
    * `dicts`: A list of dictionaries that are expected to have a key called 'type' which specifies the type of the returned value.
    """
    return list(map(deserialize_block_issuer_key, dicts))
