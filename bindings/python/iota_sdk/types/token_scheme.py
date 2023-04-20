# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

class TokenScheme():
    def __init__(self, melted_tokens=None, minted_tokens=None, maximum_supply=None):
        """Initialize TokenScheme

        Parameters
        ----------
        melted_tokens : int
        minted_tokens : int
        maximum_supply : int
        """
        self.type = 0
        self.meltedTokens = melted_tokens
        self.mintedTokens = minted_tokens
        self.maximumSupply = maximum_supply

    def as_dict(self):
        config = dict(self.__dict__)

        if 'meltedTokens' in config:
            config['meltedTokens'] = str(hex(config['meltedTokens']))
        if 'mintedTokens' in config:
            config['mintedTokens'] = str(hex(config['mintedTokens']))
        if 'maximumSupply' in config:
            config['maximumSupply'] = str(hex(config['maximumSupply']))

        return config
