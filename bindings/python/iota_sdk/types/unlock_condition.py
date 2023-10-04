# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from enum import IntEnum
from dataclasses import dataclass, field
from typing import Dict, List, TypeAlias, Union, Any
from dataclasses_json import config
from iota_sdk.types.address import AddressUnion, AccountAddress
from iota_sdk.types.common import json, SlotIndex
from iota_sdk.types.address import deserialize_address


class UnlockConditionType(IntEnum):
    """Unlock condition variants.

    Attributes:
        Address (0): An address unlock condition.
        StorageDepositReturn (1): A storage deposit return unlock condition.
        Timelock (2): A timelock unlock condition.
        Expiration (3): An expiration unlock condition.
        StateControllerAddress (4): A state controller address unlock condition.
        GovernorAddress (5): A governor address unlock condition.
        ImmutableAccountAddress (6): An immutable account address unlock condition.
    """
    Address = 0
    StorageDepositReturn = 1
    Timelock = 2
    Expiration = 3
    StateControllerAddress = 4
    GovernorAddress = 5
    ImmutableAccountAddress = 6


@json
@dataclass
class UnlockCondition():
    """Base class for unlock conditions.
    """
    type: int


@json
@dataclass
class AddressUnlockCondition(UnlockCondition):
    """An address unlock condition.

    Args:
        address: An address unlocked with a private key.
    """
    address: AddressUnion = field(
        metadata=config(
            decoder=deserialize_address
        ))
    type: int = field(
        default_factory=lambda: int(
            UnlockConditionType.Address),
        init=False)


@json
@dataclass
class StorageDepositReturnUnlockCondition(UnlockCondition):
    """A storage-deposit-return unlock condition.
    Args:
        amount: The amount of base coins the consuming transaction must deposit to `return_address`.
        return_address: The address to return the amount to.
    """
    amount: str
    return_address: AddressUnion = field(
        metadata=config(
            decoder=deserialize_address
        ))
    type: int = field(default_factory=lambda: int(
        UnlockConditionType.StorageDepositReturn), init=False)


@json
@dataclass
class TimelockUnlockCondition(UnlockCondition):
    """A timelock unlock condition.
    Args:
        slot_index: The slot index that determines when the associated output expires.
    """
    slot_index: SlotIndex
    type: int = field(
        default_factory=lambda: int(
            UnlockConditionType.Timelock),
        init=False)


@json
@dataclass
class ExpirationUnlockCondition(UnlockCondition):
    """An expiration unlock condition.
    Args:
        slot_index: The slot index that determines when the associated output expires.
        return_address: The return address if the output was not claimed in time.
    """
    slot_index: SlotIndex
    return_address: AddressUnion = field(
        metadata=config(
            decoder=deserialize_address
        ))
    type: int = field(
        default_factory=lambda: int(
            UnlockConditionType.Expiration),
        init=False)


@json
@dataclass
class StateControllerAddressUnlockCondition(UnlockCondition):
    """A state controller address unlock condition.
    Args:
        address: The state controller address that owns the output.
    """
    address: AddressUnion = field(
        metadata=config(
            decoder=deserialize_address
        ))
    type: int = field(default_factory=lambda: int(
        UnlockConditionType.StateControllerAddress), init=False)


@json
@dataclass
class GovernorAddressUnlockCondition(UnlockCondition):
    """A governor address unlock condition.
    Args:
        address: The governor address that owns the output.
    """
    address: AddressUnion = field(
        metadata=config(
            decoder=deserialize_address
        ))
    type: int = field(default_factory=lambda: int(
        UnlockConditionType.GovernorAddress), init=False)


@json
@dataclass
class ImmutableAccountAddressUnlockCondition(UnlockCondition):
    """An immutable account address unlock condition.
    Args:
        address: The permanent account address that owns this output.
    """
    address: AccountAddress
    type: int = field(default_factory=lambda: int(
        UnlockConditionType.ImmutableAccountAddress), init=False)


UnlockConditionUnion: TypeAlias = Union[AddressUnlockCondition, StorageDepositReturnUnlockCondition, TimelockUnlockCondition,
                                        ExpirationUnlockCondition, StateControllerAddressUnlockCondition, GovernorAddressUnlockCondition, ImmutableAccountAddressUnlockCondition]


def deserialize_unlock_condition(d: Dict[str, Any]) -> UnlockConditionUnion:
    """
    Takes a dictionary as input and returns an instance of a specific class based on the value of the 'type' key in the dictionary.

    Arguments:
    * `d`: A dictionary that is expected to have a key called 'type' which specifies the type of the returned value.
    """
    # pylint: disable=too-many-return-statements
    uc_type = d['type']
    if uc_type == UnlockConditionType.Address:
        return AddressUnlockCondition.from_dict(d)
    if uc_type == UnlockConditionType.StorageDepositReturn:
        return StorageDepositReturnUnlockCondition.from_dict(d)
    if uc_type == UnlockConditionType.Timelock:
        return TimelockUnlockCondition.from_dict(d)
    if uc_type == UnlockConditionType.Expiration:
        return ExpirationUnlockCondition.from_dict(d)
    if uc_type == UnlockConditionType.StateControllerAddress:
        return StateControllerAddressUnlockCondition.from_dict(d)
    if uc_type == UnlockConditionType.GovernorAddress:
        return GovernorAddressUnlockCondition.from_dict(d)
    if uc_type == UnlockConditionType.ImmutableAccountAddress:
        return ImmutableAccountAddressUnlockCondition.from_dict(d)
    raise Exception(f'invalid unlock condition type: {uc_type}')


def deserialize_unlock_conditions(
        dicts: List[Dict[str, Any]]) -> List[UnlockConditionUnion]:
    """
    Takes a list of dictionaries as input and returns a list with specific instances of a classes based on the value of the 'type' key in the dictionary.

    Arguments:
    * `dicts`: A list of dictionaries that are expected to have a key called 'type' which specifies the type of the returned value.
    """
    return list(map(deserialize_unlock_condition, dicts))
