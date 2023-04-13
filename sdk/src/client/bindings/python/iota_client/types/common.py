from enum import Enum


class Node():
    def __init__(self, url=None, jwt=None, username=None, password=None, disabled=None):
        """Initialize a Node

        Parameters
        ----------
        url : string
            Node url
        jwt : string
            JWT token
        username : string
            Username for basic authentication
        password : string
            Password for basic authentication
        disabled : bool
            Disable node
        """
        self.url = url
        self.jwt = jwt
        self.username = username
        self.password = password
        self.disabled = disabled

    def as_dict(self):
        config = {k: v for k, v in self.__dict__.items() if v != None}

        if 'jwt' in config or 'username' in config or 'password' in config:
            config['auth'] = {}
            if 'jwt' in config:
                config['auth']['jwt'] = config.pop('jwt')
            if 'username' in config or 'password' in config:
                basic_auth = config['auth']['basic_auth_name_pwd'] = []
                if 'username' in config:
                    basic_auth.append(config.pop('username'))
                if 'password' in config:
                    basic_auth.append(config.pop('password'))

        return config


class CoinType(Enum):
    IOTA = 4218
    SHIMMER = 4219


class NativeToken():
    def __init__(self, id, amount):
        """Initialise NativeToken

        Parameters
        ----------
        id : string
            Id of the token
        amount : int
            Native token amount
        """
        self.id = id
        self.amount = amount

    def as_dict(self):
        config = dict(self.__dict__)

        config['amount'] = str(hex(config['amount']))

        return config


class TokenScheme():
    def __init__(self, melted_tokens=None, minted_tokens=None, maximum_supply=None):
        """Initialise TokenScheme

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
            config['melted_tokens'] = str(hex(config['melted_tokens']))
        if 'minted_tokens' in config:
            config['minted_tokens'] = str(hex(config['minted_tokens']))
        if 'maximum_supply' in config:
            config['maximum_supply'] = str(hex(config['maximum_supply']))

        return config
