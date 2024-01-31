# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from dataclasses import dataclass, field
from enum import IntEnum
from typing import Dict, List, TypeAlias, Union, Any
from dataclasses_json import config
from iota_sdk.types.signature import Ed25519Signature
from iota_sdk.types.common import json


class UnlockType(IntEnum):
    """Unlock variants.

    Attributes:
        Signature (0): An unlock holding a signature unlocking one or more inputs.
        Reference (1): An unlock which must reference a previous unlock which unlocks also the input at the same index as this Reference Unlock.
        Account (2): An unlock which must reference a previous unlock which unlocks the account that the input is locked to.
        Anchor (3): An unlock which must reference a previous unlock which unlocks the anchor that the input is locked to.
        Nft (4): An unlock which must reference a previous unlock which unlocks the NFT that the input is locked to.
        Multi (5): Unlocks a MultiAddress with a list of other unlocks.
        Empty (6): Used to maintain correct index relationship between addresses and signatures when unlocking a MultiUnlock where not all addresses are unlocked.

    """
    Signature = 0
    Reference = 1
    Account = 2
    Anchor = 3
    Nft = 4
    Multi = 5
    Empty = 6


@json
@dataclass
class SignatureUnlock:
    """An unlock holding a signature unlocking one or more inputs.
    """
    type: int = field(
        default_factory=lambda: int(
            UnlockType.Signature),
        init=False)
    signature: Ed25519Signature


@json
@dataclass
class ReferenceUnlock:
    """An unlock which must reference a previous unlock which unlocks also the input at the same index as this Reference Unlock.
    """
    type: int = field(
        default_factory=lambda: int(
            UnlockType.Reference),
        init=False)
    reference: int


@json
@dataclass
class AccountUnlock:
    """An unlock which must reference a previous unlock which unlocks the account that the input is locked to.
    """
    type: int = field(
        default_factory=lambda: int(
            UnlockType.Account),
        init=False)
    reference: int


@json
@dataclass
class AnchorUnlock:
    """An unlock which must reference a previous unlock which unlocks the anchor that the input is locked to.
    """
    type: int = field(
        default_factory=lambda: int(
            UnlockType.Anchor),
        init=False)
    reference: int


@json
@dataclass
class NftUnlock:
    """An unlock which must reference a previous unlock which unlocks the NFT that the input is locked to.
    """
    type: int = field(default_factory=lambda: int(UnlockType.Nft), init=False)
    reference: int


# pylint: disable=missing-function-docstring,unused-argument
def deserialize_unlocks(dicts: List[Dict[str, Any]]) -> List[Unlock]:
    # Function gets overwritten further below, but needs to be defined here
    # already
    pass


@json
@dataclass
class MultiUnlock:
    """Unlocks a MultiAddress with a list of other unlocks.
    """
    type: int = field(
        default_factory=lambda: int(
            UnlockType.Multi),
        init=False)
    unlocks: List[Unlock] = field(metadata=config(
        decoder=deserialize_unlocks
    ))


@json
@dataclass
class EmptyUnlock:
    """Used to maintain correct index relationship between addresses and signatures when unlocking a MultiUnlock where not all addresses are unlocked.
    """
    type: int = field(
        default_factory=lambda: int(
            UnlockType.Empty),
        init=False)


Unlock: TypeAlias = Union[SignatureUnlock,
                          ReferenceUnlock,
                          AccountUnlock,
                          AnchorUnlock,
                          NftUnlock,
                          MultiUnlock,
                          EmptyUnlock]

# pylint: disable=too-many-return-statements


def deserialize_unlock(d: Dict[str, Any]) -> Unlock:
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
    if unlock_type == UnlockType.Anchor:
        return AnchorUnlock.from_dict(d)
    if unlock_type == UnlockType.Nft:
        return NftUnlock.from_dict(d)
    if unlock_type == UnlockType.Multi:
        return MultiUnlock.from_dict(d)
    if unlock_type == UnlockType.Empty:
        return EmptyUnlock.from_dict(d)
    raise Exception(f'invalid unlock type: {unlock_type}')

# pylint: disable=function-redefined


def deserialize_unlocks(dicts: List[Dict[str, Any]]) -> List[Unlock]:
    """
    Takes a list of dictionaries as input and returns a list with specific instances of classes based on the value of the 'type' key in the dictionary.

    Arguments:
    * `dicts`: A list of dictionaries that are expected to have a key called 'type' which specifies the type of the returned value.
    """
    return list(map(deserialize_unlock, dicts))
