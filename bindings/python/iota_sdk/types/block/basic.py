# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from dataclasses import dataclass, field
from typing import List, Optional, Union
from iota_sdk.types.block.block import Block, BlockType
from iota_sdk.types.common import HexStr, json
from iota_sdk.types.payload import TaggedDataPayload, TransactionPayload


@json
@dataclass
class BasicBlock(Block):
    """A `BasicBlock` is the most common type of block used to issue various kinds of payloads such as transactions
    at the cost of burning Mana.

    Attributes:
        strong_parents: Blocks that are strongly directly approved.
        weak_parents: Blocks that are weakly directly approved.
        shallow_like_parents: Blocks that are directly referenced to adjust opinion.
        max_burned_mana: The amount of Mana the Account identified by the IssuerId is at most willing to burn for this block.
        payload: The optional payload of this block.
    """
    strong_parents: List[HexStr]
    weak_parents: List[HexStr]
    shallow_like_parents: List[HexStr]
    max_burned_mana: str
    payload: Optional[Union[TaggedDataPayload,
                      TransactionPayload]] = None
    type: int = field(
        default_factory=lambda: BlockType.Basic,
        init=False)
