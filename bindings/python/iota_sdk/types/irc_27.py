# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from iota_sdk.types.address import Ed25519Address, AliasAddress, NFTAddress
from iota_sdk.types.common import HexStr
from iota_sdk import utf8_to_hex
from dataclasses import dataclass, field
from enum import IntEnum
from typing import Union, Optional, List

@dataclass
class Irc27Metadata:
    """The IRC27 NFT standard schema.
    Attributes:
        version: The metadata spec version.
        type: The media type (MIME) of the asset.
            Examples:
            - Image files: `image/jpeg`, `image/png`, `image/gif`, etc.
            - Video files: `video/x-msvideo` (avi), `video/mp4`, `video/mpeg`, etc.
            - Audio files: `audio/mpeg`, `audio/wav`, etc.
            - 3D Assets: `model/obj`, `model/u3d`, etc.
            - Documents: `application/pdf`, `text/plain`, etc.
        uri: URL pointing to the NFT file location.
        name: The human-readable name of the native token.
        collection_name: The human-readable collection name of the native token.
        royalties: Royalty payment addresses mapped to the payout percentage.
        issuer_name: The human-readable name of the native token creator.
        description: The human-readable description of the token.
        attributes: Additional attributes which follow [OpenSea Metadata standards](https://docs.opensea.io/docs/metadata-standards).
    """
    version: str = field(default="v1.0", init=False)
    type: str
    uri: str
    name: str
    collection_name: Optional[str]
    royalties: Optional[dict[str, float]]
    issuer_name: Optional[str]
    description: Optional[str]
    attributes: Optional[List[Attribute]]

    def as_hex(self):
        utf8_to_hex(json.dumps(self.as_dict(), , separators=(',', ':')))


@dataclass
class Attribute:
    """An attribute which follows [OpenSea Metadata standards](https://docs.opensea.io/docs/metadata-standards).
    Attributes:
        trait_type: The trait type.
        value: The value of the specified Attribute.
        display_type: The optional type used to display the Attribute.
    """
    trait_type: str
    value
    display_type: Optional[str]
