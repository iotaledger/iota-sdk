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
        self.melted_tokens = melted_tokens
        self.minted_tokens = minted_tokens
        self.maximum_supply = maximum_supply

    def as_dict(self):
        config = dict(self.__dict__)

        if 'melted_tokens' in config:
            config['meltedTokens'] = str(hex(config['melted_tokens']))
        if 'minted_tokens' in config:
            config['mintedTokens'] = str(hex(config['minted_tokens']))
        if 'maximum_supply' in config:
            config['maximumSupply'] = str(hex(config['maximum_supply']))

        return config
