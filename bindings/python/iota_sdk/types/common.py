# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from typing import NewType, Optional
from enum import IntEnum
from dataclasses import dataclass
from dataclasses_json import DataClassJsonMixin, dataclass_json, LetterCase, Undefined

HexStr = NewType("HexStr", str)


def json(cls):
    """Decorator to add custom to_dict and to_json methods to a dataclass."""
    # Apply the dataclass_json decorator first to get the default behavior
    cls = dataclass_json(letter_case=LetterCase.CAMEL, undefined=Undefined.RAISE)(cls)

    # Store original methods
    original_to_dict = cls.to_dict

    # Override methods
    def custom_to_dict(self, *args, **kwargs):
        original_dict = original_to_dict(self, *args, **kwargs)
        result = {k: v for k, v in original_dict.items() if v is not None}
        if hasattr(cls, "_to_dict_custom"):
            result = getattr(cls, "_to_dict_custom")(result)
        return result

    def custom_to_json(self, *args, **kwargs):
        # Use the custom to_dict method for serialization
        return DataClassJsonMixin.to_json(self, *args, **kwargs)

    setattr(cls, "to_dict", custom_to_dict)
    setattr(cls, "to_json", custom_to_json)

    return cls


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


@json
@dataclass
class Node():
    """Represents a node in the network.

        Attributes:
            url: The node url.
            jwt: A JWT token for authentication.
            username: A username for basic authentication.
            password: A password for basic authentication.
            disabled: Whether the node should be used for API requests or not.
    """

    url: Optional[str] = None
    jwt: Optional[str] = None
    username: Optional[str] = None
    password: Optional[str] = None
    disabled: Optional[bool] = None

    @staticmethod
    def _to_dict(encoded):

        if 'jwt' in encoded or 'username' in encoded or 'password' in encoded:
            encoded['auth'] = {}
            if 'jwt' in encoded:
                encoded['auth']['jwt'] = encoded.pop('jwt')
            if 'username' in encoded or 'password' in encoded:
                basic_auth = encoded['auth']['basicAuthNamePwd'] = []
                if 'username' in encoded:
                    basic_auth.append(encoded.pop('username'))
                if 'password' in encoded:
                    basic_auth.append(encoded.pop('password'))

        return encoded


@json
@dataclass
class AddressAndAmount():
    """Parameters to send a certain amount of coins to an address.

     Attributes:
            amount: The base coin amount to send.
            address: The receive address.
    """
    amount: int
    address: str

    def __init__(self, amount: int, address: str):
        """Initialize AddressAndAmount for options in Client::build_and_post_block()

        """
        self.amount = amount
        self.address = address

    @staticmethod
    def _to_dict_custom(config):
        config = super().to_dict()

        if 'amount' in config:
            config['amount'] = str(config['amount'])

        return config
