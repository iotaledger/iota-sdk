# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

class TokenScheme():
    def __init__(self, minted_tokens=None, melted_tokens=None, , maximum_supply=None):
        """Initialize TokenScheme
        
        Parameters
        ----------
        minted_tokens : int
        melted_tokens : int
        maximum_supply : int
        """
        self.type = 0
        
        self.mintedTokens = minted_tokens
        self.meltedTokens = melted_tokens
        self.maximumSupply = maximum_supply

    def as_dict(self):
        config = dict(self.__dict__)
        
        if 'mintedTokens' in config:
            config['mintedTokens'] = str(hex(config['mintedTokens']))
        if 'meltedTokens' in config:
            config['meltedTokens'] = str(hex(config['meltedTokens']))
        if 'maximumSupply' in config:
            config['maximumSupply'] = str(hex(config['maximumSupply']))

        return config
