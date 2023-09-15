# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from typing import List, Optional
from dataclasses import dataclass
from iota_sdk.types.block import Block
from iota_sdk.types.common import HexStr, json
from iota_sdk.types.output import OutputWithMetadata
from iota_sdk.types.output_id import OutputId
from iota_sdk.types.common import CoinType


@json
@dataclass
class Range:
    """Represents a range of address indexes.

    Attributes:
        start: The start index of the address range.
        end: The end index of the address range.
    """
    start: int
    end: int


@json
@dataclass
class GenerateAddressOptions():
    """Options for generating an address.

    Attributes:
        internal: Whether to generate an internal address.
        ledger_nano_prompt: Whether to display the generated address on Ledger Nano devices.
    """
    internal: bool
    ledger_nano_prompt: bool


@json
@dataclass
class GenerateAddressesOptions():
    """Options for generating addresses.

    Attributes:
        coin_type: The type of coin.
        range: The range of addresses to generate.
        bech32_hrp: The bech32 HRP (human readable part) to use.
        account_index: An account index.
        options: An instance of `GenerateAddressOptions`.
    """
    coin_type: CoinType
    range: Range
    bech32_hrp: str
    account_index: Optional[int] = None
    options: Optional[GenerateAddressOptions] = None


class HighLevelAPI():
    """High level API.
    """

    def get_outputs(
            self, output_ids: List[OutputId]) -> List[OutputWithMetadata]:
        """Fetch OutputWithMetadata from provided OutputIds (requests are sent in parallel).

        Args:
            output_ids: A list of output ids.

        Returns:
            A list of corresponding `OutputWithMetadata` objects.
        """
        outputs = self._call_method('getOutputs', {
            'outputIds': list(map(lambda o: o.output_id, output_ids))
        })
        return [OutputWithMetadata.from_dict(o) for o in outputs]

    def get_outputs_ignore_errors(
            self, output_ids: List[OutputId]) -> List[OutputWithMetadata]:
        """Try to get OutputWithMetadata from provided OutputIds.
        Requests are sent in parallel and errors are ignored, can be useful for spent outputs.

        Args:
            output_ids: A list of output ids.

        Returns:
            A list of corresponding `OutputWithMetadata` objects.
        """
        outputs = self._call_method('getOutputsIgnoreErrors', {
            'outputIds': list(map(lambda o: o.output_id, output_ids))
        })
        return [OutputWithMetadata.from_dict(o) for o in outputs]

    def find_blocks(self, block_ids: List[HexStr]) -> List[Block]:
        """Find all blocks by provided block IDs.

        Args:
            block_ids: A list of block ids.

        Returns:
            A list of corresponding `Block`s.
        """
        blocks = self._call_method('findBlocks', {
            'blockIds': block_ids
        })
        return [Block.from_dict(block) for block in blocks]

    def find_inputs(self, addresses: List[str], amount: int):
        """Function to find inputs from addresses for a provided amount(useful for offline signing).

        Args:
            addresses: A list of included addresses.
            amount: The amount to find inputs for.
        """
        return self._call_method('findInputs', {
            'addresses': addresses,
            'amount': amount
        })
