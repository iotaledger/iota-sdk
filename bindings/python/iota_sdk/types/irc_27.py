# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

import json
from dataclasses import dataclass, field, asdict
from typing import Optional, List, Any, Dict
from dacite import from_dict
from iota_sdk import utf8_to_hex, MetadataFeature


@dataclass
class Attribute:
    """An attribute which follows [OpenSea Metadata standards](https://docs.opensea.io/docs/metadata-standards).
    Attributes:
        trait_type: The trait type.
        value: The value of the specified Attribute.
        display_type: The optional type used to display the Attribute.
    """

    trait_type: str
    value: Any
    display_type: Optional[str] = None


@dataclass
class Irc27Metadata:
    """The IRC27 NFT standard schema.
    Attributes:
        standard: The metadata standard (IRC27).
        version: The metadata spec version (v1.0).
        type: The media type (MIME) of the asset.
            Examples:
            - Image files: `image/jpeg`, `image/png`, `image/gif`, etc.
            - Video files: `video/x-msvideo` (avi), `video/mp4`, `video/mpeg`, etc.
            - Audio files: `audio/mpeg`, `audio/wav`, etc.
            - 3D Assets: `model/obj`, `model/u3d`, etc.
            - Documents: `application/pdf`, `text/plain`, etc.
        uri: URL pointing to the NFT file location.
        name: The human-readable name of the native token.
        collectionName: The human-readable collection name of the native token.
        royalties: Royalty payment addresses mapped to the payout percentage.
        issuerName: The human-readable name of the native token creator.
        description: The human-readable description of the token.
        attributes: Additional attributes which follow [OpenSea Metadata standards](https://docs.opensea.io/docs/metadata-standards).
    """

    standard: str = field(default="IRC27", init=False)
    version: str = field(default="v1.0", init=False)
    type: str
    uri: str
    name: str
    collectionName: Optional[str] = None
    royalties: Dict[str, float] = field(default_factory=dict)
    issuerName: Optional[str] = None
    description: Optional[str] = None
    attributes: List[Attribute] = field(default_factory=list)

    @staticmethod
    def from_dict(metadata_dict: dict):
        """
        Takes a dictionary as input and returns an instance of the `Irc27Metadata` class.
        """
        return from_dict(Irc27Metadata, metadata_dict)

    def as_hex(self):
        """Turns this schema into a hex encoded string
        """
        utf8_to_hex(json.dumps(asdict(self), separators=(",", ":")))

    def as_feature(self):
        """Turns this schema into a MetadataFeature type
        """
        MetadataFeature({'irc-27': self.as_hex()})
