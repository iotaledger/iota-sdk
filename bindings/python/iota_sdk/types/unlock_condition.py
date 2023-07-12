# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from iota_sdk.types.address import Address
from enum import IntEnum
from typing import Optional
from dataclasses import dataclass


class UnlockConditionType(IntEnum):
    Address = 0
    StorageDepositReturn = 1
    Timelock = 2
    Expiration = 3
    StateControllerAddress = 4
    GovernorAddress = 5
    ImmutableAliasAddress = 6


@dataclass
class UnlockCondition():
    type: int
    amount: Optional[str] = None
    address: Optional[Address] = None
    unixTime: Optional[int] = None
    returnAddress: Optional[Address] = None

    def as_dict(self):
        config = {k: v for k, v in self.__dict__.items() if v != None}

        if 'address' in config:
            config['address'] = config['address'].as_dict()

        if 'returnAddress' in config:
            config['returnAddress'] = config['returnAddress'].as_dict()

        return config


class AddressUnlockCondition(UnlockCondition):
    def __init__(self, address):
        """Initialize an AddressUnlockCondition

        Parameters
        ----------
        address : Address
            Address
        """
        super().__init__(type=UnlockConditionType.Address, address=address)


class StorageDepositReturnUnlockCondition(UnlockCondition):
    def __init__(self, amount, return_address):
        """Initialize a StorageDepositReturnUnlockCondition

        Parameters
        ----------
        amount : int
            Amount
        return_address : Address
            Return address
        """
        super().__init__(type=UnlockConditionType.StorageDepositReturn,
                         amount=str(amount), returnAddress=return_address)


class TimelockUnlockCondition(UnlockCondition):
    def __init__(self, unix_time):
        """Initialize a TimelockUnlockCondition

        Parameters
        ----------
        unix_time : int
            Unix timestamp at which to unlock output
        """
        super().__init__(type=UnlockConditionType.Timelock, unixTime=unix_time)


class ExpirationUnlockCondition(UnlockCondition):
    def __init__(self, unix_time, return_address):
        """Initialize an ExpirationUnlockCondition

        Parameters
        ----------
        unix_time : int
            Unix timestamp
        return_address : Address
            Return address
        """
        super().__init__(type=UnlockConditionType.Expiration,
                         unixTime=unix_time, returnAddress=return_address)


class StateControllerAddressUnlockCondition(UnlockCondition):
    def __init__(self, address):
        """Initialize a StateControllerAddressUnlockCondition

        Parameters
        ----------
        address : Address
            Address for unlock condition
        """
        super().__init__(type=UnlockConditionType.StateControllerAddress, address=address)


class GovernorAddressUnlockCondition(UnlockCondition):
    def __init__(self, address):
        """Initialize a GovernorAddressUnlockCondition

        Parameters
        ----------
        address : Address
            Address for unlock condition
        """
        super().__init__(type=UnlockConditionType.GovernorAddress, address=address)


class ImmutableAliasAddressUnlockCondition(UnlockCondition):
    def __init__(self, address):
        """Initialize an ImmutableAliasAddressUnlockCondition

        Parameters
        ----------
        address : Address
            Address for unlock condition
        """
        super().__init__(type=UnlockConditionType.ImmutableAliasAddress, address=address)
