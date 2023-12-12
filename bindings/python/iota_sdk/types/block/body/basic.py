# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from dataclasses import dataclass, field
from typing import List, Optional
from dataclasses_json import config
from iota_sdk.types.block.body.type import BlockBodyType
from iota_sdk.types.common import HexStr, json
from iota_sdk.types.payload import Payload


@json
@dataclass
class BasicBlockBody:
    """A Basic Block Body is the most common type of block body used to issue various kinds of payloads such as transactions
    at the cost of burning Mana.

    Attributes:
        strong_parents: Blocks that are strongly directly approved.
        weak_parents: Blocks that are weakly directly approved.
        shallow_like_parents: Blocks that are directly referenced to adjust opinion.
        max_burned_mana: The amount of Mana the Account identified by the AccountId is at most willing to burn for this block.
        payload: The optional payload of this block.
    """
    strong_parents: List[HexStr]
    max_burned_mana: int = field(metadata=config(
        encoder=str
    ))
    weak_parents: Optional[List[HexStr]] = None
    shallow_like_parents: Optional[List[HexStr]] = None
    payload: Optional[Payload] = None
    type: int = field(
        default_factory=lambda: int(BlockBodyType.Basic),
        init=False)
