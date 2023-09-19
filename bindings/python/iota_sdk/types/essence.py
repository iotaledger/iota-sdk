# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from enum import IntEnum
from typing import Optional, List, Union

from dataclasses import dataclass, field

from iota_sdk.types.common import HexStr, json
from iota_sdk.types.output import BasicOutput, AccountOutput, FoundryOutput, NftOutput
from iota_sdk.types.input import UtxoInput
from iota_sdk.types.payload import TaggedDataPayload
from iota_sdk.types.unlock import SignatureUnlock, ReferenceUnlock


class EssenceType(IntEnum):
    """Block payload types.

    Attributes:
        RegularTransactionEssence (2): A Regular Transaction Essence.
    """
    RegularTransactionEssence = 5


@json
@dataclass
class TransactionEssence:
    type: int


@json
@dataclass
class RegularTransactionEssence(TransactionEssence):
    """Describes the essence data making up a transaction by defining its inputs, outputs, and an optional payload.

    Attributes:
        network_id: The unique value denoting whether the block was meant for mainnet, shimmer, testnet, or a private network.
                    It consists of the first 8 bytes of the BLAKE2b-256 hash of the network name.
        creation_slot: The slot index in which the transaction was created.
        context_inputs: The inputs that provide additional contextual information for the execution of a transaction.
        inputs: The inputs to consume in order to fund the outputs of the Transaction Payload.
        inputs_commitment: BLAKE2b-256 hash serving as a commitment to the serialized outputs referenced by Inputs.
        outputs: The outputs that are created by the Transaction Payload
        allotments: The allotments of Mana which which will be added upon commitment of the slot.
        payload: An optional tagged data payload
    """
    network_id: str
    # TODO: Replace with a proper SlotIndex type
    creation_slot: HexStr
    context_inputs: Optional[List[Union[CommitmentInput | BlockIssuanceCreditInput | RewardInput]]] = None
    inputs: List[UtxoInput]
    inputs_commitment: HexStr
    outputs: List[Union[BasicOutput | AccountOutput | FoundryOutput | NftOutput | DelegationOutput]]
    allotments: Optional[List[Allotment]] = None
    payload: Optional[TaggedDataPayload] = None
    type: int = field(default_factory=lambda: EssenceType.RegularTransactionEssence, init=False)
