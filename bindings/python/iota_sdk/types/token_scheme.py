# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from dataclasses import dataclass, field
from iota_sdk.types.common import HexStr, json


@json
@dataclass
class TokenScheme():
    """Base class of a token scheme.
    """
    type: int


@json
@dataclass
class SimpleTokenScheme(TokenScheme):
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

    @staticmethod
    def to_dict_custom(config):

        if isinstance(config['mintedTokens'], int):
            config['mintedTokens'] = str(hex(config['mintedTokens']))
        if isinstance(config['meltedTokens'], int):
            config['meltedTokens'] = str(hex(config['meltedTokens']))
        if isinstance(config['maximumSupply'], int):
            config['maximumSupply'] = str(hex(config['maximumSupply']))

        return config
