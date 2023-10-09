# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from dataclasses import dataclass, field
from enum import IntEnum
from typing import Dict, List, Union, Any
from iota_sdk.types.signature import Ed25519Signature
from iota_sdk.types.common import json


class UnlockType(IntEnum):
    """Unlock variants.

    Attributes:
        Signature (0): An unlock holding a signature unlocking one or more inputs.
        Reference (1): An unlock which must reference a previous unlock which unlocks also the input at the same index as this Reference Unlock.
        Account (2): An unlock which must reference a previous unlock which unlocks the account that the input is locked to.
        Nft (3): An unlock which must reference a previous unlock which unlocks the NFT that the input is locked to.
    """
    Signature = 0
    Reference = 1
    Account = 2
    Nft = 3


@json
@dataclass
class Unlock:
    """Unlock type.
    """
    type: int


@json
@dataclass
class SignatureUnlock(Unlock):
    """An unlock holding a signature unlocking one or more inputs.
    """
    signature: Ed25519Signature
    type: int = field(
        default_factory=lambda: int(
            UnlockType.Signature),
        init=False)


@json
@dataclass
class ReferenceUnlock(Unlock):
    """An unlock which must reference a previous unlock which unlocks also the input at the same index as this Reference Unlock.
    """
    reference: int
    type: int = field(
        default_factory=lambda: int(
            UnlockType.Reference),
        init=False)


@json
@dataclass
class AccountUnlock:
    """An unlock which must reference a previous unlock which unlocks the account that the input is locked to.
    """
    reference: int
    type: int = field(
        default_factory=lambda: int(
            UnlockType.Account),
        init=False)


@json
@dataclass
class NftUnlock:
    """An unlock which must reference a previous unlock which unlocks the NFT that the input is locked to.
    """
    reference: int
    type: int = field(default_factory=lambda: int(UnlockType.Nft), init=False)


def deserialize_unlock(d: Dict[str, Any]) -> Union[SignatureUnlock,
                                                   ReferenceUnlock, AccountUnlock, NftUnlock]:
    """
    Takes a dictionary as input and returns an instance of a specific class based on the value of the 'type' key in the dictionary.

    Arguments:
    * `d`: A dictionary that is expected to have a key called 'type' which specifies the type of the returned value.
    """
    unlock_type = d['type']
    if unlock_type == UnlockType.Signature:
        return SignatureUnlock.from_dict(d)
    if unlock_type == UnlockType.Reference:
        return ReferenceUnlock.from_dict(d)
    if unlock_type == UnlockType.Account:
        return AccountUnlock.from_dict(d)
    if unlock_type == UnlockType.Nft:
        return NftUnlock.from_dict(d)
    raise Exception(f'invalid unlock type: {unlock_type}')


def deserialize_unlocks(dicts: List[Dict[str, Any]]) -> List[Union[SignatureUnlock,
                                                                   ReferenceUnlock, AccountUnlock, NftUnlock]]:
    """
    Takes a list of dictionaries as input and returns a list with specific instances of a classes based on the value of the 'type' key in the dictionary.

    Arguments:
    * `dicts`: A list of dictionaries that are expected to have a key called 'type' which specifies the type of the returned value.
    """
    return list(map(deserialize_unlock, dicts))
