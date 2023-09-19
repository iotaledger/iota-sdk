# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

import json
from iota_sdk.types.common import HexStr
from iota_sdk import utf8_to_hex
from dataclasses import dataclass, field
from typing import Optional


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

    def as_hex(self):
        utf8_to_hex(json.dumps(self.as_dict(), separators=(",", ":")))
