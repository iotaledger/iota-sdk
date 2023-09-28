# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from dataclasses import dataclass
from typing import Dict
from dacite import from_dict
from iota_sdk.types.block.block import Block
from iota_sdk.types.common import HexStr, json
from iota_sdk.types.signature import Signature
from iota_sdk.utils import Utils


@json
@dataclass
class BlockWrapper:
    """The two block types introduced in this TIP share some common fields which are contained in an outer wrapper.
    This Block Wrapper is only introduced as a way to deduplicate the definition of those fields. It is not a standalone block type.
    All fields of a Block Wrapper except for the Block and Signature constitute the Block Header.
    """

    def __init__(self, header: BlockHeader, block: Block,
                 signature: Signature):
        if not isinstance(block, Block):
            raise ValueError(
                "block must be an instance of Block or its subclasses")
        self.header = header
        self.block = block
        self.signature = signature

    def id(self) -> HexStr:
        """Returns the block ID as a hexadecimal string.
        """
        return Utils.block_id(self)

    @classmethod
    def from_dict(cls, block_wrapper_dict: Dict) -> BlockWrapper:
        """
        The function `from_dict` takes a dictionary that contains the data needed to
        create an instance of the `BlockWrapper` class.

        Returns:

        An instance of the `BlockWrapper` class.
        """
        return from_dict(BlockWrapper, block_wrapper_dict)


@json
@dataclass
class BlockHeader:
    """TODO

    Attributes:
        protocol_version: Protocol version of the network to which this block belongs.
        network_id: The identifier of the network to which this block belongs.
        issuing_time: The time at which the block was issued. It is a Unix timestamp in nanoseconds.
        slot_commitment_id: The identifier of the slot to which this block commits.
        latest_finalized_slot: The slot index of the latest finalized slot.
        issuer_id: The identifier of the account that issued this block.
    """
    protocol_version: int
    network_id: str
    issuing_time: str
    slot_commitment_id: HexStr
    latest_finalized_slot: int
    issuer_id: HexStr
