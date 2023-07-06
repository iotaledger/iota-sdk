# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from enum import IntEnum
from typing import NewType

HexStr = NewType("HexStr", str)

HD_WALLET_TYPE = 44
HARDEN_MASK = 1 << 31;

class CoinType(IntEnum):
    IOTA = 4218
    SHIMMER = 4219
    ETHER = 60

    def __int__(self):
        return self.value


class Node():
    def __init__(self, url=None, jwt=None, username=None, password=None, disabled=None):
        """Initialize a Node

        Parameters
        ----------
        url : string
            Node url
        jwt : string
            JWT token
        username : string
            Username for basic authentication
        password : string
            Password for basic authentication
        disabled : bool
            Disable node
        """
        self.url = url
        self.jwt = jwt
        self.username = username
        self.password = password
        self.disabled = disabled

    def as_dict(self):
        config = {k: v for k, v in self.__dict__.items() if v != None}

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


class SendParams():
    def __init__(self, address, amount):
        """Initialize SendParams

        Parameters
        ----------
        address : string
            Address of the output
        amount : int
            Amount of the output
        """
        self.address = address
        self.amount = amount

    def as_dict(self):
        config = {k: v for k, v in self.__dict__.items() if v != None}

        if 'amount' in config:
            config['amount'] = str(config['amount'])

        return config
