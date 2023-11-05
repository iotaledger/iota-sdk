# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from dataclasses import dataclass, field
from typing import List
from iota_sdk.types.block.block import BlockType
from iota_sdk.types.common import HexStr, json


@json
@dataclass
class ValidationBlock:
    """A Validation Block is a special type of block used by validators to secure the network. It is recognized by the
    Congestion Control of the IOTA 2.0 protocol and can be issued without burning Mana within the constraints of the
    allowed validator throughput. It is allowed to reference more parent blocks than a normal Basic Block.

    Attributes:
        strong_parents: Blocks that are strongly directly approved.
        weak_parents: Blocks that are weakly directly approved.
        shallow_like_parents: Blocks that are directly referenced to adjust opinion.
        highest_supported_version: The highest supported protocol version the issuer of this block supports.
        protocol_parameters_hash: The hash of the protocol parameters for the Highest Supported Version.
    """
    strong_parents: List[HexStr]
    weak_parents: List[HexStr]
    shallow_like_parents: List[HexStr]
    highest_supported_version: int
    protocol_parameters_hash: HexStr
    type: int = field(
        default_factory=lambda: int(BlockType.Validation),
        init=False)
