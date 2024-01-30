# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from typing import TypeAlias
from dataclasses import dataclass, field
from dataclasses_json import config
from iota_sdk.types.common import hex_str_decoder, json


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
    type: int = field(default=0, init=False)
    minted_tokens: int = field(metadata=config(
        encoder=hex,
        decoder=hex_str_decoder,
    ))
    melted_tokens: int = field(metadata=config(
        encoder=hex,
        decoder=hex_str_decoder,
    ))
    maximum_supply: int = field(metadata=config(
        encoder=hex,
        decoder=hex_str_decoder,
    ))


TokenScheme: TypeAlias = SimpleTokenScheme
