# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

import json
from iota_sdk import utf8_to_hex
from dataclasses import dataclass, field
from typing import Optional, List, Any


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

    standard = field(default="IRC27", init=False)
    version: str = field(default="v1.0", init=False)
    type: str
    uri: str
    name: str
    collectionName: Optional[str] = None
    royalties: Optional[dict[str, float]] = None
    issuerName: Optional[str] = None
    description: Optional[str] = None
    attributes: Optional[List[Attribute]] = None

    def as_hex(self):
        utf8_to_hex(json.dumps(self.as_dict(), separators=(",", ":")))
