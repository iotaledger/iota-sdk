from enum import Enum


class CoinType(Enum):
    IOTA = 4218
    SHIMMER = 4219

    def __int__(self):
        return self.value


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


class AddressWithAmount():
    def __init__(self, address, amount):
        """Initialise an AddressWithAmount

        Parameters
        ----------
        address : string
            Address of the output
        amount : int
            Amount of the output
        """
        self.address = address
        self.amount = amount

    def as_dict(self):
        config = {k: v for k, v in self.__dict__.items() if v != None}

        if 'amount' in config:
            config['amount'] = str(config['amount'])

        return config
