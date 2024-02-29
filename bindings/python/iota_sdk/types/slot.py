# Copyright 2024 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from dataclasses import dataclass, field
from dataclasses_json import config
from iota_sdk.types.common import HexStr, IdWithSlotIndex, json


@json
@dataclass
class SlotCommitment:
    """Contains a summary of a slot.
    It is linked to the commitment of the previous slot, which forms a commitment chain.

    Attributes:
        protocol_version: The version of the protocol running.
        slot: The slot index of this commitment. It is calculated based on genesis timestamp and the duration of a slot.
        previous_commitment_id: The commitment ID of the previous slot.
        roots_id: The digest of multiple sparse merkle tree roots of this slot.
        cumulative_weight: The sum of previous slot commitment cumulative weight and weight of issuers of accepted blocks within this slot. It is just an indication of "committed into" this slot, and can not strictly be used for evaluating the switching of a chain.
        reference_mana_cost: Reference Mana Cost (RMC) to be used in the slot with index at `index + Max Committable Age`.
    """
    protocol_version: int
    slot: int
    previous_commitment_id: HexStr
    roots_id: HexStr
    cumulative_weight: int = field(metadata=config(
        encoder=str
    ))
    reference_mana_cost: int = field(metadata=config(
        encoder=str
    ))


class SlotCommitmentId(IdWithSlotIndex):
    """Represents a slot commitment ID.
    """
