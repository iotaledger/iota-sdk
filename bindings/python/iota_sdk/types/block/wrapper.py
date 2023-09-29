# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from dataclasses import dataclass
from typing import Union
from iota_sdk.types.block.basic import BasicBlock
from iota_sdk.types.block.validation import ValidationBlock
from iota_sdk.types.common import HexStr, json
from iota_sdk.types.signature import Ed25519Signature
from iota_sdk.utils import Utils


@json
@dataclass
class BlockWrapper:
    """A block wrapper type that can hold either a `BasicBlock` or a `ValidationBlock`.
    Shared data is stored alongside such a block in the `BlockHeader` and `Signature` fields.

    Attributes:
        header: The Block header.
        block: Holds either a `BasicBlock` or a `ValidationBlock`.
        signature: The Block signature.
    """
    protocol_version: int
    network_id: str
    issuing_time: str
    slot_commitment_id: HexStr
    latest_finalized_slot: int
    issuer_id: HexStr
    block: Union[BasicBlock, ValidationBlock]
    signature: Ed25519Signature

    def id(self) -> HexStr:
        """Returns the block ID as a hexadecimal string.
        """
        return Utils.block_id(self)
