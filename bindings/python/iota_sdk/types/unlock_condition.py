# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from enum import IntEnum

from dataclasses import dataclass, field
from typing import Union

from iota_sdk.types.address import Ed25519Address, AccountAddress, NFTAddress
from iota_sdk.types.common import json


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


@dataclass
class AddressUnlockCondition(UnlockCondition):
    """An address unlock condition.

    Args:
        address: An address unlocked with a private key.
    """
    address: Union[Ed25519Address, AccountAddress, NFTAddress]
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
    return_address: Union[Ed25519Address, AccountAddress, NFTAddress]
    type: int = field(default_factory=lambda: int(
        UnlockConditionType.StorageDepositReturn), init=False)


@json
@dataclass
class TimelockUnlockCondition(UnlockCondition):
    """A timelock unlock condition.
    Args:
        unix_time: The Unix timestamp marking the end of the timelock.
    """
    unix_time: int
    type: int = field(
        default_factory=lambda: int(
            UnlockConditionType.Timelock),
        init=False)


@json
@dataclass
class ExpirationUnlockCondition(UnlockCondition):
    """An expiration unlock condition.
    Args:
        unix_time: Unix timestamp marking the expiration of the claim.
        return_address: The return address if the output was not claimed in time.
    """
    unix_time: int
    return_address: Union[Ed25519Address, AccountAddress, NFTAddress]
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
    address: Union[Ed25519Address, AccountAddress, NFTAddress]
    type: int = field(default_factory=lambda: int(
        UnlockConditionType.StateControllerAddress), init=False)


@json
@dataclass
class GovernorAddressUnlockCondition(UnlockCondition):
    """A governor address unlock condition.
    Args:
        address: The governor address that owns the output.
    """
    address: Union[Ed25519Address, AccountAddress, NFTAddress]
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
