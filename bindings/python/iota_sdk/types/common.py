# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from enum import IntEnum
from typing import NewType

HexStr = NewType("HexStr", str)


class CoinType(IntEnum):
    """Coin types.

    Attributes:
        IOTA (4218): IOTA
        SHIMMER (4219): SHIMMER
        ETHER (60): ETHER
    """
    IOTA = 4218
    SHIMMER = 4219
    ETHER = 60

    def __int__(self):
        return self.value


class Node():
    """Represents a node in the network.
    """

    def __init__(self, url=None, jwt=None, username=None, password=None, disabled=None):
        """Initialize a Node

        Args:
            url (str, optional): Node url
            jwt (str, optional): JWT token
            username (str, optional): Username for basic authentication
            password (str, optional): Password for basic authentication
            disabled (bool, optional): Disable node
        """
        self.url = url
        self.jwt = jwt
        self.username = username
        self.password = password
        self.disabled = disabled

    def as_dict(self):
        config = {k: v for k, v in self.__dict__.items() if v is not None}

        if 'jwt' in config or 'username' in config or 'password' in config:
            config['auth'] = {}
            if 'jwt' in config:
                config['auth']['jwt'] = config.pop('jwt')
            if 'username' in config or 'password' in config:
                basic_auth = config['auth']['basicAuthNamePwd'] = []
                if 'username' in config:
                    basic_auth.append(config.pop('username'))
                if 'password' in config:
                    basic_auth.append(config.pop('password'))

        return config


class AddressAndAmount():
    """Parameters to send a certain amount of coins to an address.
    """

    def __init__(self, amount: int, address: str):
        """Initialize AddressAndAmount for options in Client::build_and_post_block()

        Args:
            amount (int): Amount of the output
            address (str): Address of the output
        """
        self.address = address
        self.amount = amount

    def as_dict(self):
        config = {k: v for k, v in self.__dict__.items() if v is not None}

        if 'amount' in config:
            config['amount'] = str(config['amount'])

        return config
