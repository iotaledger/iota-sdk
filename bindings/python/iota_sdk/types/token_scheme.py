# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from dataclasses import dataclass, field
from iota_sdk.types.common import HexStr


@dataclass
class TokenScheme():
    """Base class of a token scheme.
    """
    type: int


@dataclass
class SimpleTokenScheme(TokenScheme):
    """A simple token scheme.

    Attributes:
        mintedTokens: The number of tokens that were minted.
        meltedTokens: The number of tokens that were melted.
        maximumSupply: The maximum supply of the token.
        type: The type code of the token scheme.
    """
    mintedTokens: HexStr
    meltedTokens: HexStr
    maximumSupply: HexStr
    type: int = field(default_factory=lambda: 0, init=False)

    def as_dict(self):
        """Converts this object to a dict.
        """
        config = dict(self.__dict__)

        if isinstance(config['mintedTokens'], int):
            config['mintedTokens'] = str(hex(config['mintedTokens']))
        if isinstance(config['meltedTokens'], int):
            config['meltedTokens'] = str(hex(config['meltedTokens']))
        if isinstance(config['maximumSupply'], int):
            config['maximumSupply'] = str(hex(config['maximumSupply']))

        return config
