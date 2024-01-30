# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from enum import IntEnum
from dataclasses import dataclass, field
from typing import Dict, List, TypeAlias, Union, Any
from dataclasses_json import config
from iota_sdk.types.address import Address, AccountAddress
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
class AddressUnlockCondition:
    """An address unlock condition.

    Args:
        address: An address unlocked with a private key.
    """
    type: int = field(
        default_factory=lambda: int(
            UnlockConditionType.Address),
        init=False)
    address: Address = field(
        metadata=config(
            decoder=deserialize_address
        ))


@json
@dataclass
class StorageDepositReturnUnlockCondition:
    """A storage-deposit-return unlock condition.
    Args:
        amount: The amount of base coins the consuming transaction must deposit to `return_address`.
        return_address: The address to return the amount to.
    """
    type: int = field(default_factory=lambda: int(
        UnlockConditionType.StorageDepositReturn), init=False)
    amount: int = field(metadata=config(
        encoder=str
    ))
    return_address: Address = field(
        metadata=config(
            decoder=deserialize_address
        ))


@json
@dataclass
class TimelockUnlockCondition:
    """Defines a slot index until which the output can not be unlocked.
    Args:
        slot_index: Slot index that defines when the output can be consumed.
    """
    type: int = field(
        default_factory=lambda: int(
            UnlockConditionType.Timelock),
        init=False)
    slot_index: SlotIndex


@json
@dataclass
class ExpirationUnlockCondition:
    """Defines a slot index until which only the Address defined in the Address Unlock Condition is allowed to unlock the output. After the slot index is reached/passed, only the Return Address can unlock it.
    Args:
        slot_index: Before this slot index, Address Unlock Condition is allowed to unlock the output,
                    after that only the address defined in Return Address.
        return_address: The return address if the output was not claimed in time.
    """
    type: int = field(
        default_factory=lambda: int(
            UnlockConditionType.Expiration),
        init=False)
    slot_index: SlotIndex
    return_address: Address = field(
        metadata=config(
            decoder=deserialize_address
        ))


@json
@dataclass
class StateControllerAddressUnlockCondition:
    """A state controller address unlock condition.
    Args:
        address: The state controller address that owns the output.
    """
    type: int = field(default_factory=lambda: int(
        UnlockConditionType.StateControllerAddress), init=False)
    address: Address = field(
        metadata=config(
            decoder=deserialize_address
        ))


@json
@dataclass
class GovernorAddressUnlockCondition:
    """A governor address unlock condition.
    Args:
        address: The governor address that owns the output.
    """
    type: int = field(default_factory=lambda: int(
        UnlockConditionType.GovernorAddress), init=False)
    address: Address = field(
        metadata=config(
            decoder=deserialize_address
        ))


@json
@dataclass
class ImmutableAccountAddressUnlockCondition:
    """An immutable account address unlock condition.
    Args:
        address: The permanent account address that owns this output.
    """
    type: int = field(default_factory=lambda: int(
        UnlockConditionType.ImmutableAccountAddress), init=False)
    address: AccountAddress


UnlockCondition: TypeAlias = Union[AddressUnlockCondition, StorageDepositReturnUnlockCondition, TimelockUnlockCondition,
                                   ExpirationUnlockCondition, StateControllerAddressUnlockCondition, GovernorAddressUnlockCondition, ImmutableAccountAddressUnlockCondition]


def deserialize_unlock_condition(d: Dict[str, Any]) -> UnlockCondition:
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
        dicts: List[Dict[str, Any]]) -> List[UnlockCondition]:
    """
    Takes a list of dictionaries as input and returns a list with specific instances of classes based on the value of the 'type' key in the dictionary.

    Arguments:
    * `dicts`: A list of dictionaries that are expected to have a key called 'type' which specifies the type of the returned value.
    """
    return list(map(deserialize_unlock_condition, dicts))
