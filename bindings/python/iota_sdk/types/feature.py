# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from enum import IntEnum

from dataclasses import dataclass, field

from iota_sdk.types.address import Ed25519Address, AliasAddress, NFTAddress
from iota_sdk.types.common import HexStr, json


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


@json
@dataclass
class Feature():
    """Base class of a feature.
    """
    type: int


@json
@dataclass
class SenderFeature(Feature):
    """Sender feature.
    Attributes:
        address: A given sender address.
    """
    address: Ed25519Address | AliasAddress | NFTAddress
    type: int = field(
        default_factory=lambda: int(
            FeatureType.Sender),
        init=False)


@json
@dataclass
class IssuerFeature(Feature):
    """Issuer feature.
    Attributes:
        address: A given issuer address.
    """
    address: Ed25519Address | AliasAddress | NFTAddress
    type: int = field(
        default_factory=lambda: int(
            FeatureType.Issuer),
        init=False)


@json
@dataclass
class MetadataFeature(Feature):
    """Metadata feature.
    Attributes:
        data: Some hex encoded metadata.
    """
    data: HexStr
    type: int = field(
        default_factory=lambda: int(
            FeatureType.Metadata),
        init=False)


@json
@dataclass
class TagFeature(Feature):
    """Tag feature.
    Attributes:
        tag: A hex encoded tag used to index the output.
    """
    tag: HexStr
    type: int = field(default_factory=lambda: int(FeatureType.Tag), init=False)
