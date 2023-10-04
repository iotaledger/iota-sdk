# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from enum import IntEnum
from typing import TYPE_CHECKING, Optional, List, TypeAlias

from dataclasses import dataclass, field

from iota_sdk.types.common import HexStr, json, SlotIndex
from iota_sdk.types.mana import ManaAllotment
from iota_sdk.types.input import UtxoInput
from iota_sdk.types.context_input import ContextInput
from iota_sdk.types.output import Output

# Required to prevent circular import
if TYPE_CHECKING:
    from iota_sdk.types.payload import Payload


class EssenceType(IntEnum):
    """Block payload types.

    Attributes:
        RegularTransactionEssence (2): A Regular Transaction Essence.
    """
    RegularTransactionEssence = 2


@json
@dataclass
class BaseTransactionEssence:
    """Base class of Transaction essence
    """
    type: int


@json
@dataclass
class RegularTransactionEssence(BaseTransactionEssence):
    """Describes the essence data making up a transaction by defining its inputs, outputs, and an optional payload.

    Attributes:
        network_id: The unique value denoting whether the block was meant for mainnet, shimmer, testnet, or a private network.
                    It consists of the first 8 bytes of the BLAKE2b-256 hash of the network name.
        creation_slot: The slot index in which the transaction was created.
        inputs: The inputs to consume in order to fund the outputs of the Transaction Payload.
        inputs_commitment: BLAKE2b-256 hash serving as a commitment to the serialized outputs referenced by Inputs.
        outputs: The outputs that are created by the Transaction Payload
        context_inputs: The inputs that provide additional contextual information for the execution of a transaction.
        allotments: The allotments of Mana which which will be added upon commitment of the slot.
        payload: An optional tagged data payload
    """
    network_id: str
    creation_slot: SlotIndex
    inputs: List[UtxoInput]
    inputs_commitment: HexStr
    outputs: List[Output]
    context_inputs: Optional[List[ContextInput]] = None
    allotments: Optional[List[ManaAllotment]] = None
    payload: Optional[Payload] = None
    type: int = field(
        default_factory=lambda: EssenceType.RegularTransactionEssence,
        init=False)


TransactionEssence: TypeAlias = RegularTransactionEssence
