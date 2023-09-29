# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from dataclasses import dataclass
from typing import Dict, Union
from dacite import from_dict
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
    signature: Union[Ed25519Signature]

    def id(self) -> HexStr:
        """Returns the block ID as a hexadecimal string.
        """
        return Utils.block_id(self)

    @classmethod
    def from_dict(cls, wrapper_dict: Dict) -> BlockWrapper:
        """
        The function `from_dict` takes a dictionary that contains the data needed to
        create an instance of the `BlockWrapper` class.

        Returns:

        An instance of the `BlockWrapper` class.
        """
        return from_dict(BlockWrapper, wrapper_dict)
