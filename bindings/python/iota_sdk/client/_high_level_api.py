# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from iota_sdk.secret_manager.secret_manager import LedgerNanoSecretManager, MnemonicSecretManager, StrongholdSecretManager, SeedSecretManager
from iota_sdk.types.common import HexStr
from iota_sdk.types.output_id import OutputId
from iota_sdk.types.common import CoinType
from typing import List, Optional


class Range:
    def __init__(self, start: int, end: int):
        self.start = start
        self.end = end


class GenerateAddressOptions():
    def __init__(self,  internal: bool, ledgerNanoPrompt: bool):
        self.internal = internal
        self.ledgerNanoPrompt = ledgerNanoPrompt


class GenerateAddressesOptions():
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
        config = {k: v for k, v in self.__dict__.items() if v != None}

        config["range"] = config["range"].__dict__
        if "options" in config:
            config["options"] = config["options"].__dict__
        return config


class HighLevelAPI():

    def get_outputs(self, output_ids: List[OutputId]):
        """Fetch OutputResponse from provided OutputIds (requests are sent in parallel).
        """
        return self._call_method('getOutputs', {
            'outputIds': output_ids
        })

    def get_outputs_ignore_errors(self, output_ids: List[OutputId]):
        """Try to get OutputResponse from provided OutputIds.
           Requests are sent in parallel and errors are ignored, can be useful for spent outputs.
        """
        return self._call_method('getOutputsIgnoreErrors', {
            'outputIds': output_ids
        })

    def find_blocks(self, block_ids: List[HexStr]):
        """Find all blocks by provided block IDs.
        """
        return self._call_method('findBlocks', {
            'blockIds': block_ids
        })

    def retry(self, block_id: HexStr):
        """Retries (promotes or reattaches) a block for provided block id. Block should only be
           retried only if they are valid and haven't been confirmed for a while.
        """
        return self._call_method('retry', {'blockId': block_id})

    def retry_until_included(self, block_id: HexStr, interval: Optional[int] = None, max_attempts: Optional[int] = None):
        """Retries (promotes or reattaches) a block for provided block id until it's included (referenced by a
           milestone). Default interval is 5 seconds and max attempts is 40. Returns the included block at first
           position and additional reattached blocks.
        """
        return self._call_method('retryUntilIncluded', {
            'blockId': block_id,
            'interval': interval,
            'maxAttempts': max_attempts
        })

    def consolidate_funds(self, secret_manager: LedgerNanoSecretManager | MnemonicSecretManager | SeedSecretManager | StrongholdSecretManager, generate_addresses_options: GenerateAddressesOptions):
        """Function to consolidate all funds from a range of addresses to the address with the lowest index in that range
           Returns the address to which the funds got consolidated, if any were available.
        """
        return self._call_method('consolidateFunds', {
            'secretManager': secret_manager,
            'generateAddressesOptions': generate_addresses_options.as_dict(),
        })

    def find_inputs(self, addresses: List[str], amount: int):
        """Function to find inputs from addresses for a provided amount (useful for offline signing)
        """
        return self._call_method('findInputs', {
            'addresses': addresses,
            'amount': amount
        })

    def find_outputs(self, output_ids: List[OutputId], addresses: List[str]):
        """Find all outputs based on the requests criteria. This method will try to query multiple nodes if
           the request amount exceeds individual node limit.
        """
        return self._call_method('findOutputs', {
            'outputIds': output_ids,
            'addresses': addresses
        })

    def reattach(self, block_id: HexStr):
        """Reattaches blocks for provided block id. Blocks can be reattached only if they are valid and haven't been
           confirmed for a while.
        """
        return self._call_method('reattach', {
            'blockId': block_id
        })

    def reattach_unchecked(self, block_id: HexStr):
        """Reattach a block without checking if it should be reattached.
        """
        return self._call_method('reattachUnchecked', {
            'blockId': block_id
        })

    def promote(self, block_id: HexStr):
        """Promotes a block. The method should validate if a promotion is necessary through get_block. If not, the
           method should error out and should not allow unnecessary promotions.
        """
        return self._call_method('promote', {
            'blockId': block_id
        })

    def promote_unchecked(self, block_id: HexStr):
        """Promote a block without checking if it should be promoted.
        """
        return self._call_method('promoteUnchecked', {
            'blockId': block_id
        })
