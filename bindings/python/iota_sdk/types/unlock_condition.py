# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from iota_sdk.types.address import Address
from enum import IntEnum
from typing import Optional
from dataclasses import dataclass


class UnlockConditionType(IntEnum):
    """The type of unlock condition.

    Attributes:
        Address (0): address unlock condition
        StorageDepositReturn (1): storage deposit return unlock condition
        Timelock (2): timelock unlock condition
        Expiration (3): expiration unlock condition
        StateControllerAddress (4): state controller address unlock condition
        GovernorAddress (5): governor address unlock condition
        ImmutableAliasAddress (6): immutable alias address unlock condition
    """
    Address = 0
    StorageDepositReturn = 1
    Timelock = 2
    Expiration = 3
    StateControllerAddress = 4
    GovernorAddress = 5
    ImmutableAliasAddress = 6


@dataclass
class UnlockCondition():
    """Base class for unlock conditions.

    Attributes:
        type (int): the type of unlock condition
    """

    type: int
    amount: Optional[str] = None
    address: Optional[Address] = None
    unixTime: Optional[int] = None
    returnAddress: Optional[Address] = None

    def as_dict(self):
        config = {k: v for k, v in self.__dict__.items() if v is not None}

        if 'address' in config:
            config['address'] = config['address'].as_dict()

        if 'returnAddress' in config:
            config['returnAddress'] = config['returnAddress'].as_dict()

        return config


class AddressUnlockCondition(UnlockCondition):
    """An address unlock condition.

    Attributes:
        address (Address): a regular address owned by an account
    """
    def __init__(self, address):
        """Initialize an AddressUnlockCondition

        Args:
            address (Address): a regular address owned by an account
        """
        super().__init__(type=UnlockConditionType.Address, address=address)


class StorageDepositReturnUnlockCondition(UnlockCondition):
    """A storage deposit return unlock condition.

    Attributes:
        amount (int): amount of coins the consuming transaction should return
        return_address (Address): the address to return the amount to
    """
    def __init__(self, amount, return_address):
        """Initialize a StorageDepositReturnUnlockCondition

        Args:
            amount (int): amount of coins the consuming transaction should return
            return_address (Address): the address to return the amount to
        """
        super().__init__(type=UnlockConditionType.StorageDepositReturn,
                         amount=str(amount), returnAddress=return_address)


class TimelockUnlockCondition(UnlockCondition):
    """A timelock unlock condition.

    Attributes:
        unix_time (int): Unix timestamp until which an output cannot be unlocked/claimed
    """
    def __init__(self, unix_time):
        """Initialize a TimelockUnlockCondition

        Args:
            unix_time (int): Unix timestamp until which an output cannot be unlocked/claimed
        """
        super().__init__(type=UnlockConditionType.Timelock, unixTime=unix_time)


class ExpirationUnlockCondition(UnlockCondition):
    """An expiration unlock condition.

    Attributes:
        unix_time (int): Unix timestamp until which an output can be unlocked/claimed
        return_address (Address): return address if the output was not unlocked/claimed in time
    """
    def __init__(self, unix_time, return_address):
        """Initialize an ExpirationUnlockCondition

        Args:
            unix_time (int): Unix timestamp until which an output can be unlocked/claimed
            return_address (Address): return address if the output was not unlocked/claimed in time
        """
        super().__init__(type=UnlockConditionType.Expiration,
                         unixTime=unix_time, returnAddress=return_address)


class StateControllerAddressUnlockCondition(UnlockCondition):
    """A state controller address unlock condition.
    """
    def __init__(self, address):
        """Initialize a StateControllerAddressUnlockCondition

        Args:
            address (Address): a state controller address
        """
        super().__init__(type=UnlockConditionType.StateControllerAddress, address=address)


class GovernorAddressUnlockCondition(UnlockCondition):
    """A governor address unlock condition.
    """
    def __init__(self, address):
        """Initialize a GovernorAddressUnlockCondition

        Args:
            address (Address): a governor address
        """
        super().__init__(type=UnlockConditionType.GovernorAddress, address=address)


class ImmutableAliasAddressUnlockCondition(UnlockCondition):
    """An immutable alias address unlock condition.
    """
    def __init__(self, address):
        """Initialize an ImmutableAliasAddressUnlockCondition

        Args:
            address (Address): an immutable alias address
        """
        super().__init__(type=UnlockConditionType.ImmutableAliasAddress, address=address)
