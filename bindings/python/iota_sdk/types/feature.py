# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from iota_sdk.types.address import Address
from iota_sdk.types.common import HexStr
from dataclasses import dataclass
from enum import IntEnum
from typing import Optional


class FeatureType(IntEnum):
    """Types of features.

    Attributes:
        Sender (0): The sender feature.
        Issuer (1): The issuer feature.
        Metadata (2): The metadata feature.
        Tag (3): The tag feature.
    """
    Sender = 0
    Issuer = 1
    Metadata = 2
    Tag = 3


@dataclass
class Feature():
    """Base class of a feature.

    Attributes:
        type: The type of feature.
        address: Holds either an issuer or a sender address.
        data: Some hex encoded metadata.
        tag: A hex encoded tag used to index an output.
    """
    type: int
    address: Optional[Address] = None
    data: Optional[HexStr] = None
    tag: Optional[HexStr] = None

    def into(self):
        """Downcast to the actual feature type.
        """
        match FeatureType(self.type):
            case FeatureType.Sender:
                return SenderFeature(self.address)
            case FeatureType.Issuer:
                return IssuerFeature(self.address)
            case FeatureType.Metadata:
                return MetadataFeature(self.data)
            case FeatureType.Tag:
                return TagFeature(self.tag)

    def as_dict(self):
        res = {k: v for k, v in self.__dict__.items() if v is not None}
        if 'address' in res:
            res['address'] = res['address'].as_dict()
        return res


class SenderFeature(Feature):
    """Sender feature.
    """

    def __init__(self, sender):
        """Initialize a SenderFeature

        Args:
            sender: A given sender address.
        """
        super().__init__(int(FeatureType.Sender), address=sender)


class IssuerFeature(Feature):
    """Issuer feature.
    """

    def __init__(self, issuer):
        """Initialize an IssuerFeature

        Args:
            issuer: A given issuer address.
        """
        super().__init__(int(FeatureType.Issuer), address=issuer)


class MetadataFeature(Feature):
    """Metadata feature.
    """

    def __init__(self, data: HexStr):
        """Initialize a MetadataFeature

        Args:
            data: Some hex encoded metadata.
        """
        super().__init__(int(FeatureType.Metadata), data=data)


class TagFeature(Feature):
    """Tag feature.
    """

    def __init__(self, tag: HexStr):
        """Initialize a TagFeature

        Args:
            tag: A hex encoded tag used to index the output.
        """
        super().__init__(int(FeatureType.Tag), tag=tag)
