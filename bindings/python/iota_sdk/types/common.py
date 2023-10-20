# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from typing import NewType, Optional
from enum import IntEnum
from dataclasses import dataclass, field
from dataclasses_json import dataclass_json, LetterCase, Undefined, config

HexStr = NewType("HexStr", str)
EpochIndex = NewType("EpochIndex", int)
SlotIndex = NewType("SlotIndex", int)


def json(cls):
    """Decorator to add to_dict and to_json methods to a dataclass."""

    # Store override methods if they exist
    override_to_dict = getattr(cls, "to_dict", None)
    override_to_json = getattr(cls, "to_json", None)

    # Apply the dataclass_json decorator to get the default behavior
    cls = dataclass_json(
        letter_case=LetterCase.CAMEL,
        undefined=Undefined.RAISE)(cls)

    # Re-apply the original fns if they exist
    if override_to_dict:
        setattr(cls, "to_dict", override_to_dict)
    if override_to_json:
        setattr(cls, "to_json", override_to_json)

    # Override to_dict to remove None values
    def custom_to_dict(cls, *args, **kwargs):
        original_dict = cls.to_dict(*args, **kwargs)
        result = {k: v for k, v in original_dict.items() if v is not None}
        return result

    setattr(cls, "to_dict", custom_to_dict)

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

    def to_dict(self):
        res = {
            'url': self.url,
            'disabled': self.disabled
        }
        if not self.jwt is None or not self.username is None or not self.password is None:
            auth = res['auth'] = {}
            if not self.jwt is None:
                auth['jwt'] = self.jwt
            if not self.username is None or not self.password is None:
                basic_auth = auth['basicAuthNamePwd'] = []
                if not self.username is None:
                    basic_auth.append(self.username)
                if not self.password is None:
                    basic_auth.append(self.password)

        return res


def opt_int_encoder(value):
    """Transforms int to string if Optional is not None

     Attributes:
            value: The optional int
    """
    if value is not None:
        return str(value)
    return None


@json
@dataclass
class AddressAndAmount():
    """Parameters to send a certain amount of coins to an address.

     Attributes:
            amount: The base coin amount to send.
            address: The receive address.
    """
    amount: int = field(metadata=config(
        encoder=str
    ))
    address: str
