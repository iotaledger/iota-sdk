# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from typing import List, Optional, Union
from abc import ABCMeta, abstractmethod
from dacite import from_dict
from iota_sdk.types.block import Block
from iota_sdk.types.common import CoinType, HexStr
from iota_sdk.types.output import OutputWithMetadata
from iota_sdk.types.output_id import OutputId
from iota_sdk.secret_manager.secret_manager import LedgerNanoSecretManager, MnemonicSecretManager, StrongholdSecretManager, SeedSecretManager


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

    # pylint: disable=redefined-builtin
    def __init__(self, coinType: CoinType,
                 range: Range,
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
        self.range = range
        self.bech32Hrp = bech32Hrp
        self.accountIndex = accountIndex
        self.options = options

    def as_dict(self):
        """Converts this object to a dict.
        """
        config = {k: v for k, v in self.__dict__.items() if v is not None}

        config["range"] = config["range"].__dict__
        if "options" in config:
            config["options"] = config["options"].__dict__
        return config


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

    def retry(self, block_id: HexStr) -> List[Union[HexStr, Block]]:
        """Retries (promotes or reattaches) a block for provided block id. Block should only be
        retried only if they are valid and haven't been confirmed for a while.

        Args:
            block_id: A block id.

        Returns:
            A list where the first element is the block id and the second one the block.
        """
        result = self._call_method('retry', {'blockId': block_id})
        result[1] = Block.from_dict(result[1])
        return result

    def retry_until_included(
            self, block_id: HexStr, interval: Optional[int] = None, max_attempts: Optional[int] = None) -> List[List[Union[HexStr, Block]]]:
        """Retries (promotes or reattaches) a block for provided block id until it's included (referenced by a
        milestone). Default interval is 5 seconds and max attempts is 40. Returns the included block at first
        position and additional reattached blocks.

        Args:
            block_id: A block id.
            interval: A retry interval in seconds. Defaults to 5.
            max_attempts: A maximum number of retries. Defaults to 40.

        Returns:
            A list of lists where the first element is the block id and the second one the block.
        """
        result = self._call_method('retryUntilIncluded', {
            'blockId': block_id,
            'interval': interval,
            'maxAttempts': max_attempts
        })

        def block_class(block_id_and_block):
            block_id_and_block[1] = Block.from_dict(block_id_and_block[1])
            return block_id_and_block
        blockIdsAndBlocks = [block_class(block_id_and_block)
                             for block_id_and_block in result]
        return blockIdsAndBlocks

    def consolidate_funds(self, secret_manager: Union[LedgerNanoSecretManager, MnemonicSecretManager, SeedSecretManager,
                          StrongholdSecretManager], generate_addresses_options: GenerateAddressesOptions) -> str:
        """Function to consolidate all funds from a range of addresses to the address with the lowest index in that range.
        Returns the address to which the funds got consolidated, if any were available.

        Args:
            secret_manager: A supported secret manager.
            generate_addresses_options: Options to generate addresses.

        Returns:
            An address to which the funds got consolidated.
        """
        return self._call_method('consolidateFunds', {
            'secretManager': secret_manager,
            'generateAddressesOptions': generate_addresses_options.as_dict(),
        })

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

    def reattach(self, block_id: HexStr) -> List[Union[HexStr, Block]]:
        """Reattaches blocks for a provided block id. Blocks can be reattached only if they are valid and
        haven't been confirmed for a while .

        Args:
            block_id: A block id of a block that should be reattached.

        Returns:
            The reattached block id and block.
        """
        result = self._call_method('reattach', {
            'blockId': block_id
        })
        result[1] = Block.from_dict(result[1])
        return result

    def reattach_unchecked(
            self, block_id: HexStr) -> List[Union[HexStr, Block]]:
        """Reattach a block without checking if it should be reattached.

        Args:
            block_id: A block id of a block that should be reattached.

        Returns:
            The reattached block id and block.
        """
        result = self._call_method('reattachUnchecked', {
            'blockId': block_id
        })
        result[1] = Block.from_dict(result[1])
        return result

    def promote(self, block_id: HexStr) -> List[Union[HexStr, Block]]:
        """Promotes a block. The method should validate if a promotion is necessary through get_block.
        If not, the method should error out and should not allow unnecessary promotions.

        Args:
            block_id: A block id of a block that should be promoted.

        Returns:
            The block id and block that promoted the provided block.
        """
        result = self._call_method('promote', {
            'blockId': block_id
        })
        result[1] = Block.from_dict(result[1])
        return result

    def promote_unchecked(
            self, block_id: HexStr) -> List[Union[HexStr, Block]]:
        """Promote a block without checking if it should be promoted.

        Args:
            block_id: A block id of a block that should be promoted.

        Returns:
            The block id and block that promoted the provided block.
        """
        result = self._call_method('promoteUnchecked', {
            'blockId': block_id
        })
        result[1] = Block.from_dict(result[1])
        return result
