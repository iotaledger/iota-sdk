# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from dataclasses import dataclass, field
from typing import TypeAlias
from iota_sdk.types.common import HexStr, json


@json
@dataclass
class SimpleTokenScheme:
    """A simple token scheme.

    Attributes:
        minted_tokens: The number of tokens that were minted.
        melted_tokens: The number of tokens that were melted.
        maximum_supply: The maximum supply of the token.
        type: The type code of the token scheme.
    """
    minted_tokens: HexStr
    melted_tokens: HexStr
    maximum_supply: HexStr
    type: int = field(default_factory=lambda: 0, init=False)


TokenScheme: TypeAlias = SimpleTokenScheme
