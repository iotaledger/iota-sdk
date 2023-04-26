# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from enum import Enum


class FeatureType(Enum):
    Sender = 0
    Issuer = 1
    Metadata = 2
    Tag = 3


class Feature():
    def __init__(self, type, sender=None, issuer=None, data=None, tag=None):
        """Initialize a feature

        Parameters
        ----------
        type : FeatureType
            The type of feature
        sender : Address
            Sender address
        issuer : Address
            Issuer Address
        data : string
            Hex encoded metadata
        tag : string
            Hex encoded tag used to index the output
        """
        self.type = type
        self.sender = sender
        self.issuer = issuer
        self.data = data
        self.tag = tag

    def as_dict(self):
        config = {k: v for k, v in self.__dict__.items() if v != None}

        config['type'] = config['type'].value

        if 'sender' in config:
            config['address'] = config.pop('sender').as_dict()

        if 'issuer' in config:
            config['address'] = config.pop('issuer').as_dict()

        return config


class SenderFeature(Feature):
    def __init__(self, sender):
        """Initialize a SenderFeature

        Parameters
        ----------
        sender : Address
            Sender address
        """
        super().__init__(FeatureType.Sender, sender=sender)


class IssuerFeature(Feature):
    def __init__(self, issuer):
        """Initialize an IssuerFeature

        Parameters
        ----------
        issuer : Address
            Issuer address
        """
        super().__init__(FeatureType.Issuer, issuer=issuer)


class MetadataFeature(Feature):
    def __init__(self, data):
        """Initialize a MetadataFeature

        Parameters
        ----------
        data : string
            Hex encoded metadata
        """
        super().__init__(FeatureType.Metadata, data=data)


class TagFeature(Feature):
    def __init__(self, tag):
        """Initialize a TagFeature

        Parameters
        ----------
        tag : string
            Hex encoded tag used to index the output
        """
        super().__init__(FeatureType.Tag, tag=tag)
