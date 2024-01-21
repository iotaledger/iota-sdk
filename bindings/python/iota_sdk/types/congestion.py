# Copyright 2024 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from typing import List
from dataclasses import dataclass, field
from dataclasses_json import config
from iota_sdk.types.common import HexStr, json
from iota_sdk.types.slot import SlotCommitment


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
    slot: int
    ready: bool
    reference_mana_cost: int = field(metadata=config(
        encoder=str
    ))
    block_issuance_credits: int = field(metadata=config(
        encoder=str
    ))


@json
@dataclass
class IssuanceBlockHeader:
    """Information that is ideal for attaching a block in the network.
    Response of GET /api/core/v3/blocks/issuance

    Attributes:
        strong_parents: Blocks that are strongly directly approved.
        weak_parents: Blocks that are weakly directly approved.
        shallow_like_parents: Blocks that are directly referenced to adjust opinion.
        latest_parent_block_issuing_time: Latest issuing time of the returned parents.
        latest_finalized_slot: The slot index of the latest finalized slot.
        latest_commitment: The latest slot commitment.
    """
    strong_parents: List[HexStr]
    weak_parents: List[HexStr]
    shallow_like_parents: List[HexStr]
    latest_parent_block_issuing_time: int = field(metadata=config(
        encoder=str
    ))
    latest_finalized_slot: int
    latest_commitment: SlotCommitment
