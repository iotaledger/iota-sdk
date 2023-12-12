# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from dataclasses import dataclass, field
from typing import TypeAlias, Union
from dataclasses_json import config
from iota_sdk.utils import Utils
from iota_sdk.types.common import HexStr, json, SlotIndex
from iota_sdk.types.node_info import ProtocolParameters
from iota_sdk.types.signature import Signature
from iota_sdk.types.block.body.basic import BasicBlockBody
from iota_sdk.types.block.body.validation import ValidationBlockBody


@json
@dataclass
class Block:
    """A block that can hold either a `BasicBlockBody` or a `ValidationBlockBody`.
    Shared data is stored alongside such a block in the header fields.

    Attributes:
        protocol_version: Protocol version of the network to which this block belongs.
        network_id: The identifier of the network to which this block belongs.
        issuing_time: The time at which the block was issued. It is a Unix timestamp in nanoseconds.
        slot_commitment_id: The identifier of the slot to which this block commits.
        latest_finalized_slot: The slot index of the latest finalized slot.
        issuer_id: The identifier of the account that issued this block.
        body: Holds either a `BasicBlockBody` or a `ValidationBlockBody`.
        signature: The Block signature.
    """
    protocol_version: int
    network_id: int = field(metadata=config(
        encoder=str
    ))
    issuing_time: int = field(metadata=config(
        encoder=str
    ))
    slot_commitment_id: HexStr
    latest_finalized_slot: SlotIndex
    issuer_id: HexStr
    body: BlockBody
    signature: Signature

    def id(self, params: ProtocolParameters) -> HexStr:
        """Returns the block ID as a hexadecimal string.
        """
        return Utils.block_id(self, params)


@json
@dataclass
class UnsignedBlock:
    """An unsigned block type that can hold either a `BasicBlock` or a `ValidationBlock`.
    Shared data is stored alongside such a block in the header fields.

    Attributes:
        protocol_version: Protocol version of the network to which this block belongs.
        network_id: The identifier of the network to which this block belongs.
        issuing_time: The time at which the block was issued. It is a Unix timestamp in nanoseconds.
        slot_commitment_id: The identifier of the slot to which this block commits.
        latest_finalized_slot: The slot index of the latest finalized slot.
        issuer_id: The identifier of the account that issued this block.
        block: Holds either a `BasicBlock` or a `ValidationBlock`.
    """
    protocol_version: int
    network_id: int = field(metadata=config(
        encoder=str
    ))
    issuing_time: int = field(metadata=config(
        encoder=str
    ))
    slot_commitment_id: HexStr
    latest_finalized_slot: SlotIndex
    issuer_id: HexStr
    body: BlockBody


BlockBody: TypeAlias = Union[BasicBlockBody, ValidationBlockBody]
