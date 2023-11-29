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
        outputs: The outputs that are created by the Transaction Payload
        payload: An optional tagged data payload
    """
    network_id: str
    creation_slot: SlotIndex
    inputs: List[UtxoInput]
    capabilities: Optional[HexStr] = field(default=None, init=False)
    outputs: List[Output]
    context_inputs: Optional[List[ContextInput]] = None
    allotments: Optional[List[ManaAllotment]] = None
    payload: Optional[Payload] = None

    def with_capabilities(self, capabilities: bytes):
        """Sets the transaction capabilities from a byte array.
        Attributes:
            capabilities: The transaction capabilities bitflags.
        """
        if any(c != 0 for c in capabilities):
            self.capabilities = '0x' + capabilities.hex()
        else:
            self.capabilities = None
