# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from typing import List, Optional
from dataclasses import dataclass
from abc import ABCMeta, abstractmethod
from iota_sdk.client.responses import OutputResponse
from iota_sdk.types.block.block import Block
from iota_sdk.types.block.id import BlockId
from iota_sdk.types.common import CoinType, json
from iota_sdk.types.output_id import OutputId


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
class GenerateAddressOptions:
    """Options for generating an address.

    Attributes:
        internal: Whether to generate an internal address.
        ledger_nano_prompt: Whether to display the generated address on Ledger Nano devices.
    """
    internal: bool
    ledger_nano_prompt: bool


@json
@dataclass
class GenerateAddressesOptions:
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


class HighLevelAPI(metaclass=ABCMeta):
    """High level API.
    """

    @abstractmethod
    def _call_method(self, name, data=None):
        """
        Sends a message to the Rust library and returns the response.
        It is abstract here as its implementation is located in `client.py`, which is a composite class.

        Arguments:

        * `name`: The `name` parameter is a string that represents the name of the method to be called.
        It is used to identify the specific method to be executed in the Rust library.
        * `data`: The `data` parameter is an optional parameter that represents additional data to be
        sent along with the method call. It is a dictionary that contains key-value pairs of data. If
        the `data` parameter is provided, it will be included in the `message` dictionary as the 'data'
        key.

        Returns:

        The method returns either the payload from the JSON response or the entire response if there is
        no payload.
        """

    def get_outputs(
            self, output_ids: List[OutputId]) -> List[OutputResponse]:
        """Fetch OutputResponse from provided OutputIds (requests are sent in parallel).

        Args:
            output_ids: A list of output ids.

        Returns:
            A list of corresponding `OutputResponse` objects.
        """
        outputs = self._call_method('getOutputs', {
            'outputIds': list(map(lambda o: o.output_id, output_ids))
        })
        return [OutputResponse.from_dict(o) for o in outputs]

    def get_outputs_ignore_not_found(
            self, output_ids: List[OutputId]) -> List[OutputResponse]:
        """Try to get OutputResponse from provided OutputIds.
        Requests are sent in parallel and errors are ignored, can be useful for spent outputs.

        Args:
            output_ids: A list of output ids.

        Returns:
            A list of corresponding `OutputResponse` objects.
        """
        outputs = self._call_method('getOutputsIgnoreNotFound', {
            'outputIds': list(map(lambda o: o.output_id, output_ids))
        })
        return [OutputResponse.from_dict(o) for o in outputs]

    def find_blocks(self, block_ids: List[BlockId]) -> List[Block]:
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
