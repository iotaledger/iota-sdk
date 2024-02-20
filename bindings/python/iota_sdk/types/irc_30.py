# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

import json
from dataclasses import dataclass, field, asdict
from typing import Optional
from dacite import from_dict
from iota_sdk.types.common import HexStr
from iota_sdk import utf8_to_hex, MetadataFeature


@dataclass
class Irc30Metadata:
    """The IRC30 native token metadata standard schema.
    Attributes:
        standard: The metadata standard (IRC30).
        name: The human-readable name of the native token.
        symbol: The symbol/ticker of the token.
        decimals: Number of decimals the token uses (divide the token amount by 10^decimals to get its user representation).
        description: The human-readable description of the token.
        url: URL pointing to more resources about the token.
        logoUrl: URL pointing to an image resource of the token logo.
        logo: The svg logo of the token encoded as a byte string.
    """

    standard: str = field(default="IRC30", init=False)
    name: str
    symbol: str
    decimals: int
    description: Optional[str] = None
    url: Optional[str] = None
    logoUrl: Optional[str] = None
    logo: Optional[HexStr] = None

    @staticmethod
    def from_dict(metadata_dict: dict):
        """
        Takes a dictionary as input and returns an instance of the `Irc30Metadata` class.
        """
        return from_dict(Irc30Metadata, metadata_dict)

    def as_hex(self):
        """Turns this schema into a hex encoded string
        """
        utf8_to_hex(json.dumps(asdict(self), separators=(",", ":")))

    def as_feature(self):
        """Turns this schema into a MetadataFeature type
        """
        MetadataFeature({'irc-30': self.as_hex()})
