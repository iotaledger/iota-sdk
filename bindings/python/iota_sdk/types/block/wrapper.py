# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from dataclasses import dataclass
from typing import Union
from iota_sdk.types.block.basic import BasicBlock
from iota_sdk.types.block.validation import ValidationBlock
from iota_sdk.types.common import HexStr, json, SlotIndex
from iota_sdk.types.signature import Ed25519Signature
from iota_sdk.utils import Utils


@json
@dataclass
class BlockWrapper:
    """A block wrapper type that can hold either a `BasicBlock` or a `ValidationBlock`.
    Shared data is stored alongside such a block in the `BlockHeader` and `Signature` fields.

    Attributes:
        protocol_version: Protocol version of the network to which this block belongs.
        network_id: The identifier of the network to which this block belongs.
        issuing_time: The time at which the block was issued. It is a Unix timestamp in nanoseconds.
        slot_commitment_id: The identifier of the slot to which this block commits.
        latest_finalized_slot: The slot index of the latest finalized slot.
        issuer_id: The identifier of the account that issued this block.
        block: Holds either a `BasicBlock` or a `ValidationBlock`.
        signature: The Block signature.
    """
    protocol_version: int
    network_id: str
    issuing_time: str
    slot_commitment_id: HexStr
    latest_finalized_slot: SlotIndex
    issuer_id: HexStr
    block: Union[BasicBlock, ValidationBlock]
    signature: Ed25519Signature

    def id(self) -> HexStr:
        """Returns the block ID as a hexadecimal string.
        """
        return Utils.block_id(self)
