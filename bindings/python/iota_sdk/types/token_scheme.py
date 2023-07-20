# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from dataclasses import dataclass
from iota_sdk.types.common import HexStr


@dataclass
class TokenScheme():
    """A token scheme.

    Attributes:
        mintedTokens: The number of tokens that were minted.
        meltedTokens: The number of tokens that were melted.
        maximumSupply: The maximum supply of the token.
        type: The type code of the token scheme.
    """
    mintedTokens: HexStr
    meltedTokens: HexStr
    maximumSupply: HexStr
    type: int = 0

    def as_dict(self):
        config = dict(self.__dict__)

        if isinstance(config['mintedTokens'], int):
            config['mintedTokens'] = str(hex(config['mintedTokens']))
        if isinstance(config['meltedTokens'], int):
            config['meltedTokens'] = str(hex(config['meltedTokens']))
        if isinstance(config['maximumSupply'], int):
            config['maximumSupply'] = str(hex(config['maximumSupply']))

        return config
