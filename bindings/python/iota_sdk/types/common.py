# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from typing import NewType, Optional
from enum import IntEnum
from dataclasses import dataclass, field
from dataclasses_json import DataClassJsonMixin, dataclass_json, LetterCase, Undefined, config

HexStr = NewType("HexStr", str)
EpochIndex = NewType("EpochIndex", int)
SlotIndex = NewType("SlotIndex", int)


def json(cls):
    """Decorator to add to_dict and to_json methods to a dataclass."""

    # Get potential override method
    to_dict = getattr(cls, "to_dict", None)

    # Apply the dataclass_json decorator to get the default behavior
    cls = dataclass_json(
        letter_case=LetterCase.CAMEL,
        undefined=Undefined.RAISE)(cls)

    # If no custom one is defined, set the default from dataclass_json
    if to_dict is None:
        to_dict = cls.to_dict

    # Override to_dict to remove None values
    def custom_to_dict(self, *args, **kwargs):
        # pylint: disable=protected-access
        original_dict = to_dict(self, *args, **kwargs)

        # recursive remove the None values
        def filter_none(value):
            if isinstance(value, dict):
                return {k: filter_none(v)
                        for k, v in value.items() if v is not None}
            if isinstance(value, list):
                return [filter_none(item)
                        for item in value if item is not None]
            return value

        return filter_none(original_dict)

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
class Node:
    """Represents a node in the network.

        Attributes:
            url: The node url.
            jwt: A JWT token for authentication.
            username: A username for basic authentication.
            password: A password for basic authentication.
            disabled: Whether the node should be used for API requests or not.
            permanode: Whether the node is a permanode or not.
    """

    url: Optional[str] = None
    jwt: Optional[str] = None
    username: Optional[str] = None
    password: Optional[str] = None
    disabled: Optional[bool] = None
    permanode: Optional[bool] = None

    def to_dict(self) -> dict:
        """Custom dict conversion.
        """

        res = {
            'url': self.url,
            'disabled': self.disabled,
            'permanode': self.permanode
        }
        if self.jwt is not None or self.username is not None or self.password is not None:
            auth = res['auth'] = {}
            if self.jwt is not None:
                auth['jwt'] = self.jwt
            if self.username is not None or self.password is not None:
                basic_auth = auth['basicAuthNamePwd'] = []
                if self.username is not None:
                    basic_auth.append(self.username)
                if self.password is not None:
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


def hex_str_decoder(value: str) -> int:
    """Parses a given string as a hexadecimal integer."""
    return int(value, 16)


@json
@dataclass
class AddressAndAmount:
    """Parameters to send a certain amount of coins to an address.

     Attributes:
            amount: The base coin amount to send.
            address: The receive address.
    """
    amount: int = field(metadata=config(
        encoder=str
    ))
    address: str


class IdWithSlotIndex(str):
    """Represents an hex encoded ID that contains a slot index at the end.

    Attributes:
        id: The hex encoded ID with a slot index.

    """

    def slot_index(self):
        """Returns the slot index of the ID.
        """
        return int.from_bytes(
            bytes.fromhex(self[-8:]), 'little')
