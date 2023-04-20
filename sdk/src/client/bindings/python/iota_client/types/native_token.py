class NativeToken():
    def __init__(self, id, amount):
        """Initialize NativeToken

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
