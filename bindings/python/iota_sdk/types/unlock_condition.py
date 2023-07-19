# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from iota_sdk.types.address import Address
from enum import IntEnum
from typing import Optional
from dataclasses import dataclass


class UnlockConditionType(IntEnum):
    """Unlock condition variants.

    Attributes:
        Address (0): An address unlock condition.
        StorageDepositReturn (1): A storage deposit return unlock condition.
        Timelock (2): A timelock unlock condition.
        Expiration (3): An expiration unlock condition.
        StateControllerAddress (4): A state controller address unlock condition.
        GovernorAddress (5): A governor address unlock condition.
        ImmutableAliasAddress (6): An immutable alias address unlock condition.
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
        type: The type code of the unlock condition.
        amount: Some amount depending on the unlock condition type.
        address: Some address depending on the unlock condition type.
        unixTime: Some Unix timestamp depending on the unlock condition type.
        return_address: An address to return funds to.
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
    """

    def __init__(self, address):
        """Initialize `Self`.

        Args:
            address: An address unlocked with a private key.
        """
        super().__init__(type=UnlockConditionType.Address, address=address)


class StorageDepositReturnUnlockCondition(UnlockCondition):
    """A storage-deposit-return unlock condition.
    """

    def __init__(self, amount, return_address):
        """Initialize `Self`.

        Args:
            amount: The amount of base coins the consuming transaction should deposit to `return_address`.
            return_address: The address to return the amount to.
        """
        super().__init__(type=UnlockConditionType.StorageDepositReturn,
                         amount=str(amount), returnAddress=return_address)


class TimelockUnlockCondition(UnlockCondition):
    """A timelock unlock condition.
    """

    def __init__(self, unix_time):
        """Initialize `Self`.

        Args:
            unix_time: The Unix timestamp marking the end of the timelock.
        """
        super().__init__(type=UnlockConditionType.Timelock, unixTime=unix_time)


class ExpirationUnlockCondition(UnlockCondition):
    """An expiration unlock condition.
    """

    def __init__(self, unix_time, return_address):
        """Initialize an ExpirationUnlockCondition

        Args:
            unix_time: Unix timestamp marking the expiration of the claim.
            return_address: The return address if the output was not claimed in time.
        """
        super().__init__(type=UnlockConditionType.Expiration,
                         unixTime=unix_time, returnAddress=return_address)


class StateControllerAddressUnlockCondition(UnlockCondition):
    """A state controller address unlock condition.
    """

    def __init__(self, address):
        """Initialize `Self`.

        Args:
            address: The state controller address that owns the output.
        """
        super().__init__(type=UnlockConditionType.StateControllerAddress, address=address)


class GovernorAddressUnlockCondition(UnlockCondition):
    """A governor address unlock condition.
    """

    def __init__(self, address):
        """Initialize `Self`.

        Args:
            address: The governor address that owns the output.
        """
        super().__init__(type=UnlockConditionType.GovernorAddress, address=address)


class ImmutableAliasAddressUnlockCondition(UnlockCondition):
    """An immutable alias address unlock condition.
    """

    def __init__(self, address):
        """Initialize `Self`.

        Args:
            address: The permanent alias address that owns this output.
        """
        super().__init__(type=UnlockConditionType.ImmutableAliasAddress, address=address)
