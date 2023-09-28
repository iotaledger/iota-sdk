# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from dataclasses import dataclass, field
from typing import Dict, List, Optional, Union
from dacite import from_dict
from iota_sdk.types.block.block import Block, BlockType
from iota_sdk.types.common import HexStr, json
from iota_sdk.types.payload import TaggedDataPayload, TransactionPayload


@json
@dataclass
class BasicBlock(Block):
    """TODO: copy description from TIP-46 once added

    Attributes:
        strong_parents: Blocks that are strongly directly approved.
        weak_parents: Blocks that are weakly directly approved.
        shallow_like_parents: Blocks that are directly referenced to adjust opinion.
        payload: The optional payload of this block.
        burned_mana: The amount of Mana the Account identified by the IssuerId is at most willing to burn for this block.
    """
    strong_parents: List[HexStr]
    weak_parents: List[HexStr]
    shallow_like_parents: List[HexStr]
    payload: Optional[Union[TaggedDataPayload,
                      TransactionPayload]] = None
    burned_mana: str
    type: int = field(
        default_factory=lambda: BlockType.Basic,
        init=False)

    @classmethod
    def from_dict(cls, basic_block_dict: Dict) -> BasicBlock:
        """
        The function `from_dict` takes a dictionary that contains the data needed to
        create an instance of the `BasicBlock` class.

        Returns:

        An instance of the `BasicBlock` class.
        """
        return from_dict(BasicBlock, basic_block_dict)
