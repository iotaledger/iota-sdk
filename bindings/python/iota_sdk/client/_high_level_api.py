# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from iota_sdk.types.common import HexStr
from iota_sdk.types.output import OutputWithMetadata
from iota_sdk.types.output_id import OutputId
from iota_sdk.types.common import CoinType
from typing import List, Optional
from dacite import from_dict


class Range:
    """Represents a range of address indexes.

    Attributes:
        start: The start index of the address range.
        end: The end index of the address range.
    """

    def __init__(self, start: int, end: int):
        self.start = start
        self.end = end


class GenerateAddressOptions():
    """Options for generating an address.

    Attributes:
        internal: Whether to generate an internal address.
        ledgerNanoPrompt: Whether to display the generated address on Ledger Nano devices.
    """

    def __init__(self, internal: bool, ledgerNanoPrompt: bool):
        """Initialize GenerateAddressOptions.
        """
        self.internal = internal
        self.ledgerNanoPrompt = ledgerNanoPrompt


class GenerateAddressesOptions():
    """Options for generating addresses.

    Attributes:
        coinType: The type of coin.
        range: The range of addresses to generate.
        bech32Hrp: The bech32 HRP (human readable part) to use.
        accountIndex: An account index.
        options: An instance of `GenerateAddressOptions`.
    """

    def __init__(self, coinType: CoinType,
                 range: range,
                 bech32Hrp: str,
                 accountIndex: Optional[int] = None,
                 options: Optional[GenerateAddressOptions] = None):
        """Initialize GenerateAddressesOptions.

        Args:
            coinType: The type of coin.
            range: The range of addresses to generate.
            bech32Hrp: The bech32 HRP (human readable part) to use.
            accountIndex: An account index.
            options: An instance of `GenerateAddressOptions`.
        """
        self.coinType = coinType
        self.range = Range(range.start, range.stop)
        self.bech32Hrp = bech32Hrp
        self.accountIndex = accountIndex
        self.options = options

    def as_dict(self):
        config = {k: v for k, v in self.__dict__.items() if v is not None}

        config["range"] = config["range"].__dict__
        if "options" in config:
            config["options"] = config["options"].__dict__
        return config


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
        return [from_dict(OutputWithMetadata, o) for o in outputs]

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
        return [from_dict(OutputWithMetadata, o) for o in outputs]

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
