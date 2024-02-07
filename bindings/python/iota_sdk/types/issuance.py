# Copyright 2024 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from typing import List, Optional
from dataclasses import dataclass, field
from dataclasses_json import config
from iota_sdk.types.common import HexStr, json, SlotIndex
from iota_sdk.types.slot import SlotCommitment


@json
@dataclass
class IssuanceBlockHeader:
    """Information that is used to attach a block in the network.
    Response of GET /api/core/v3/blocks/issuance

    Attributes:
        strong_parents: Blocks that are strongly directly approved.
        latest_parent_block_issuing_time: Latest issuing time of the returned parents.
        latest_finalized_slot: The slot index of the latest finalized slot.
        latest_commitment: The latest slot commitment.
        weak_parents: Blocks that are weakly directly approved.
        shallow_like_parents: Blocks that are directly referenced to adjust opinion.
    """
    strong_parents: List[HexStr]
    latest_parent_block_issuing_time: int = field(metadata=config(
        encoder=str
    ))
    latest_finalized_slot: SlotIndex
    latest_commitment: SlotCommitment
    weak_parents: Optional[List[HexStr]] = None
    shallow_like_parents: Optional[List[HexStr]] = None


@json
@dataclass
class Congestion:
    """Provides the cost and readiness to issue estimates.
    Response of GET /api/core/v3/accounts/{accountId}/congestion.

    Attributes:
        slot: The slot index for which the congestion estimate is provided.
        ready: Indicates if a node is ready to issue a block in a current congestion or should wait.
        reference_mana_cost: The cost in mana for issuing a block in a current congestion estimated based on RMC and slot index.
        block_issuance_credits: The Block Issuance Credits of the requested account.
    """
    slot: SlotIndex
    ready: bool
    reference_mana_cost: int = field(metadata=config(
        encoder=str
    ))
    block_issuance_credits: int = field(metadata=config(
        encoder=str
    ))
