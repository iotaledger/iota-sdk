# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from iota_sdk.secret_manager.secret_manager import LedgerNanoSecretManager, MnemonicSecretManager, StrongholdSecretManager, SeedSecretManager
from iota_sdk.types.block import Block
from iota_sdk.types.common import HexStr
from iota_sdk.types.output import OutputWithMetadata
from iota_sdk.types.output_id import OutputId
from iota_sdk.types.common import CoinType
from typing import List, Optional
from dacite import from_dict


class Range:
    """Represents a range of address indexes.

    Attributes:
        start: the start index of the address range
        end: the end index of the address range
    """

    def __init__(self, start: int, end: int):
        self.start = start
        self.end = end


class GenerateAddressOptions():
    """Options for generating an address.

    Attributes:
        internal: whether to generate internal addresses
        ledgerNanoPrompt: whether to prompt the user for a ledger nano seed
    """

    def __init__(self, internal: bool, ledgerNanoPrompt: bool):
        """Initialize GenerateAddressOptions.
        """
        self.internal = internal
        self.ledgerNanoPrompt = ledgerNanoPrompt


class GenerateAddressesOptions():
    """Options for generating addresses.

    Attributes:
        coinType (CoinType): the coin type
        range (Range): the range
        bech32Hrp (str): the bech32 HRP
        accountIndex (int, optional): the account index
        options (GenerateAddressOptions, optional): generate address options
    """

    def __init__(self, coinType: CoinType,
                 range: range,
                 bech32Hrp: str,
                 accountIndex: Optional[int] = None,
                 options: Optional[GenerateAddressOptions] = None):
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
            output_ids (List[OutputId]): list of output ids

        Returns:
            list of `OutputWithMetadata` objects
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
            output_ids (List[OutputId]): list of output ids

        Returns:
            list of `OutputWithMetadata` objects
        """
        outputs = self._call_method('getOutputsIgnoreErrors', {
            'outputIds': list(map(lambda o: o.output_id, output_ids))
        })
        return [from_dict(OutputWithMetadata, o) for o in outputs]

    def find_blocks(self, block_ids: List[HexStr]) -> List[Block]:
        """Find all blocks by provided block IDs.

        Args:
            block_ids (List[HexStr]): list of block ids

        Returns:
            list of `Block` objects
        """
        blocks = self._call_method('findBlocks', {
            'blockIds': block_ids
        })
        return [Block.from_dict(block) for block in blocks]

    def retry(self, block_id: HexStr) -> List[HexStr | Block]:
        """Retries (promotes or reattaches) a block for provided block id. Block should only be
        retried only if they are valid and haven't been confirmed for a while.

        Args:
            block_id (HexStr): block id

        Returns:
            list of `HexStr` or `Block` objects
        """
        result = self._call_method('retry', {'blockId': block_id})
        result[1] = Block.from_dict(result[1])
        return result

    def retry_until_included(
            self, block_id: HexStr, interval: Optional[int] = None, max_attempts: Optional[int] = None) -> List[List[HexStr | Block]]:
        """Retries (promotes or reattaches) a block for provided block id until it's included (referenced by a
        milestone). Default interval is 5 seconds and max attempts is 40. Returns the included block at first
        position and additional reattached blocks.

        Args:
            block_id (HexStr): block id
            interval (int, optional): retry interval in seconds. Defaults to 5.
            max_attempts (int, optional): maximum number of retries. Defaults to 40.

        Returns:
            list of a list of `HexStr` or `Block` objects
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


    def consolidate_funds(self, secret_manager: LedgerNanoSecretManager | MnemonicSecretManager | SeedSecretManager |
                          StrongholdSecretManager, generate_addresses_options: GenerateAddressesOptions) -> str:
        """Function to consolidate all funds from a range of addresses to the address with the lowest index in that range.
        Returns the address to which the funds got consolidated, if any were available.

        Args:
            secret_manager(LedgerNanoSecretManager | MnemonicSecretManager | SeedSecretManager | StrongholdSecretManager): a supported secret manager
            generate_addresses_options(GenerateAddressesOptions): options to generate addresses

        Returns:
            str: address to which the funds got consolidated
        """
        return self._call_method('consolidateFunds', {
            'secretManager': secret_manager,
            'generateAddressesOptions': generate_addresses_options.as_dict(),
        })

    def find_inputs(self, addresses: List[str], amount: int):
        """Function to find inputs from addresses for a provided amount(useful for offline signing).

        Args:
            addresses(List[str]): list of included addresses
            amount(int): amount to find
        """
        return self._call_method('findInputs', {
            'addresses': addresses,
            'amount': amount
        })

    def find_outputs(self, output_ids: List[OutputId], addresses: List[str]):
        """Find all outputs based on the requests criteria. This method will try to query multiple nodes if
        the request amount exceeds individual node limit.

        Args:
            output_ids(List[OutputId]): list of included output ids
            addresses(List[str]): list of included addresses
        """
        return self._call_method('findOutputs', {
            'outputIds': output_ids,
            'addresses': addresses
        })

    def reattach(self, block_id: HexStr) -> List[HexStr | Block]:
        """Reattaches blocks for a provided block id. Blocks can be reattached only if they are valid and
        haven't been confirmed for a while .

        Args:
            block_id(HexStr): block id to reattach

        Returns:
            list of `HexStr` or `Block` objects
        """
        result = self._call_method('reattach', {
            'blockId': block_id
        })
        result[1] = Block.from_dict(result[1])
        return result

    def reattach_unchecked(self, block_id: HexStr) -> List[HexStr | Block]:
        """Reattach a block without checking if it should be reattached.

        Args:
            block_id(HexStr): block id to reattach

        Returns:
            list of `HexStr` or `Block` objects
        """
        result = self._call_method('reattachUnchecked', {
            'blockId': block_id
        })
        result[1] = Block.from_dict(result[1])
        return result

    def promote(self, block_id: HexStr) -> List[HexStr | Block]:
        """Promotes a block. The method should validate if a promotion is necessary through get_block.
        If not, the method should error out and should not allow unnecessary promotions.

        Args:
            block_id(HexStr): block id to promote

        Returns:
            list of `HexStr` or `Block` objects
        """
        result = self._call_method('promote', {
            'blockId': block_id
        })
        result[1] = Block.from_dict(result[1])
        return result

    def promote_unchecked(self, block_id: HexStr) -> List[HexStr | Block]:
        """Promote a block without checking if it should be promoted.

        Args:
            block_id(HexStr): block id to promote

        Returns:
            list of `HexStr` or `Block` objects
        """
        result = self._call_method('promoteUnchecked', {
            'blockId': block_id
        })
        result[1] = Block.from_dict(result[1])
        return result
