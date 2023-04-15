from enum import Enum


class UnlockConditionType(Enum):
    Address = 0
    StorageDepositReturn = 1
    Timelock = 2
    Expiration = 3
    StateControllerAddress = 4
    GovernorAddress = 5
    ImmutableAliasAddress = 6


class UnlockCondition():
    def __init__(self, type=None, address=None, amount=None, unix_time=None, return_address=None):
        """Initialize an UnlockCondition

        Parameters
        ----------
        type : UnlockConditionType
            The type of unlock condition
        address : Address
            Address for unlock condition
        amount : int
            Amount for storage deposit unlock condition
        unix_time : int
            Unix timestamp for timelock and expiration unlock condition
        return_address : Address
            Return address for expiration and storage deposit unlock condition
        """
        self.type = type
        self.address = address
        self.amount = amount
        self.unix_time = unix_time
        self.return_address = return_address

    def as_dict(self):
        config = {k: v for k, v in self.__dict__.items() if v != None}

        if 'type' in config:
            config['type'] = config['type'].value

        if 'address' in config:
            config['address'] = config['address'].as_dict()

        if 'return_address' in config:
            config['return_address'] = config['return_address'].as_dict()

        if 'amount' in config:
            config['amount'] = str(config['amount'])

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
                         amount=amount, return_address=return_address)


class TimelockUnlockCondition(UnlockCondition):
    def __init__(self, unix_time):
        """Initialize a TimelockUnlockCondition

        Parameters
        ----------
        unix_time : int
            Unix timestamp at which to unlock output
        """
        super().__init__(type=UnlockConditionType.Timelock, unix_time=unix_time)


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
                         unix_time=unix_time, return_address=return_address)


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
