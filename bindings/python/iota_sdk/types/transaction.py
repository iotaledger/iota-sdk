# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from typing import TYPE_CHECKING, Optional, List

from dataclasses import dataclass, field

from iota_sdk.types.common import HexStr, json, SlotIndex
from iota_sdk.types.mana import ManaAllotment
from iota_sdk.types.input import UtxoInput
from iota_sdk.types.context_input import ContextInput
from iota_sdk.types.output import Output

# Required to prevent circular import
if TYPE_CHECKING:
    from iota_sdk.types.payload import Payload


@json
@dataclass
class Transaction:
    """A transaction consuming inputs, creating outputs and carrying an optional payload.

    Attributes:
        network_id: The unique value denoting whether the block was meant for mainnet, shimmer, testnet, or a private network.
                    It consists of the first 8 bytes of the BLAKE2b-256 hash of the network name.
        creation_slot: The slot index in which the transaction was created.
        context_inputs: The inputs that provide additional contextual information for the execution of a transaction.
        inputs: The inputs to consume in order to fund the outputs of the Transaction Payload.
        allotments: The allotments of Mana which which will be added upon commitment of the slot.
        capabilities: The capability bitflags of the transaction.
        payload: An optional tagged data payload
        outputs: The outputs that are created by the Transaction Payload
    """
    network_id: str
    creation_slot: SlotIndex
    context_inputs: List[ContextInput]
    inputs: List[UtxoInput]
    allotments: List[ManaAllotment]
    capabilities: HexStr = field(default='0x', init=False)
    payload: Optional[Payload] = None
    outputs: List[Output]

    def with_capabilities(self, capabilities: bytes):
        """Sets the transaction capabilities from a byte array.
        Attributes:
            capabilities: The transaction capabilities bitflags.
        """
        self.capabilities = '0x' + capabilities.hex()
