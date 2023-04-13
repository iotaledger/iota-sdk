from enum import Enum

class AddressType(Enum):
    ED25519 = 0
    ALIAS = 8
    NFT = 16

class Address():
    def __init__(self, type, address_or_id):
        """Initialize an Address
        
        Parameters
        ----------
        type : AddressType
            The type of the Address
        address_or_id : string
            The address to use. Can either be an hex encoded ED25519 address or NFT/Alias id
        """
        self.type = type
        self.address_or_id = address_or_id

    def as_dict(self):
        config = dict(self.__dict__)

        config['type'] = config['type'].value
        
        if self.type == AddressType.ED25519:
            config['pubKeyHash'] = config.pop('address_or_id')
        elif self.type == AddressType.ALIAS:
            config['alias_id'] = config.pop('address_or_id')
        elif self.type == AddressType.NFT:
            config['nft_id'] = config.pop('address_or_id')

        return config

class Ed25519Address(Address):
    def __init__(self, address):
        """Initialize an Ed25519Address
        
        Parameters
        ----------
        address : string
            The hex encoded address to use.
        """
        super().__init__(AddressType.ED25519, address)

class AliasAddress(Address):
    def __init__(self, address_or_id):
        """Initialize an AliasAddress
        
        Parameters
        ----------
        address_or_id : string
            The hex encoded address to use.
        """
        super().__init__(AddressType.ALIAS, address_or_id)

class NFTAddress(Address):
    def __init__(self, address_or_id):
        """Initialize an NFTokenAddress
        
        Parameters
        ----------
        address_or_id : string
            The hex encoded address to use.
        """
        super().__init__(AddressType.NFT, address_or_id)
