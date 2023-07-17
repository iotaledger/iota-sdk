# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from iota_sdk.types.address import Address
from iota_sdk.types.common import HexStr
from dataclasses import dataclass
from enum import IntEnum
from typing import Optional


class FeatureType(IntEnum):
    Sender = 0
    Issuer = 1
    Metadata = 2
    Tag = 3

@dataclass
class Feature():
    """Initialize a feature

    Parameters
    ----------
    type : FeatureType
        The type of feature
    address : Address
        Issuer or Sender address
    data : HexStr
        Hex encoded metadata
    tag : HexStr
        Hex encoded tag used to index the output
    """
    type: int
    address: Optional[Address] = None
    data: Optional[HexStr] = None
    tag: Optional[HexStr] = None

    def into(self):
        match FeatureType(self.type):
            case FeatureType.Sender:
                return SenderFeature(self.address)
            case FeatureType.Issuer:
                return IssuerFeature(self.address)
            case FeatureType.Metadata:
                return MetadataFeature(self.data)
            case FeatureType.Metadata:
                return TagFeature(self.tag)
        
    def as_dict(self):
        res = {k: v for k, v in self.__dict__.items() if v != None}
        if 'address' in res:
            res['address'] = res['address'].as_dict()
        return res


class SenderFeature(Feature):
    def __init__(self, sender):
        """Initialize a SenderFeature

        Parameters
        ----------
        sender : Address
            Sender address
        """
        super().__init__(int(FeatureType.Sender), address=sender)


class IssuerFeature(Feature):
    def __init__(self, issuer):
        """Initialize an IssuerFeature

        Parameters
        ----------
        issuer : Address
            Issuer address
        """
        super().__init__(int(FeatureType.Issuer), address=issuer)


class MetadataFeature(Feature):
    def __init__(self, data: HexStr):
        """Initialize a MetadataFeature

        Parameters
        ----------
        data : HexStr
            Hex encoded metadata
        """
        super().__init__(int(FeatureType.Metadata), data=data)


class TagFeature(Feature):
    def __init__(self, tag: HexStr):
        """Initialize a TagFeature

        Parameters
        ----------
        tag : HexStr
            Hex encoded tag used to index the output
        """
        super().__init__(int(FeatureType.Tag), tag=tag)
