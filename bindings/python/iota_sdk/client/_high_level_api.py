# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from iota_sdk.client._base_api import BaseAPI


class HighLevelAPI(BaseAPI):

    def get_outputs(self, output_ids):
        """Fetch OutputResponse from provided OutputIds (requests are sent in parallel).
        """
        return self.call_method('getOutputs', {
            'outputIds': output_ids
        })

    def try_get_outputs(self, output_ids):
        """Try to get OutputResponse from provided OutputIds.
           Requests are sent in parallel and errors are ignored, can be useful for spent outputs.
        """
        return self.call_method('tryGetOutputs', {
            'outputIds': output_ids
        })

    def find_blocks(self, block_ids):
        """Find all blocks by provided block IDs.
        """
        return self.call_method('findBlocks', {
            'blockIds': block_ids
        })

    def retry(self, block_id):
        """Retries (promotes or reattaches) a block for provided block id. Block should only be
           retried only if they are valid and haven't been confirmed for a while.
        """
        return self.call_method('retry', {'blockId': block_id})

    def retry_until_included(self, block_id, interval=None, max_attempts=None):
        """Retries (promotes or reattaches) a block for provided block id until it's included (referenced by a
           milestone). Default interval is 5 seconds and max attempts is 40. Returns the included block at first
           position and additional reattached blocks.
        """
        return self.call_method('retryUntilIncluded', {
            'blockId': block_id,
            'interval': interval,
            'maxAttempts': max_attempts
        })

    def consolidate_funds(self, secret_manager, generate_addresses_options):
        """Function to consolidate all funds from a range of addresses to the address with the lowest index in that range
           Returns the address to which the funds got consolidated, if any were available.
        """
        return self.call_method('consolidateFunds', {
            'secretManager': secret_manager,
            'generateAddressesOptions': generate_addresses_options,
        })

    def find_inputs(self, addresses, amount):
        """Function to find inputs from addresses for a provided amount (useful for offline signing)
        """
        return self.call_method('findInputs', {
            'addresses': addresses,
            'amount': amount
        })

    def find_outputs(self, output_ids, addresses):
        """Find all outputs based on the requests criteria. This method will try to query multiple nodes if
           the request amount exceeds individual node limit.
        """
        return self.call_method('findOutputs', {
            'outputIds': output_ids,
            'addresses': addresses
        })

    def reattach(self, block_id):
        """Reattaches blocks for provided block id. Blocks can be reattached only if they are valid and haven't been
           confirmed for a while.
        """
        return self.call_method('reattach', {
            'blockId': block_id
        })

    def reattach_unchecked(self, block_id):
        """Reattach a block without checking if it should be reattached.
        """
        return self.call_method('reattachUnchecked', {
            'blockId': block_id
        })

    def promote(self, block_id):
        """Promotes a block. The method should validate if a promotion is necessary through get_block. If not, the
           method should error out and should not allow unnecessary promotions.
        """
        return self.call_method('promote', {
            'blockId': block_id
        })

    def promote_unchecked(self, block_id):
        """Promote a block without checking if it should be promoted.
        """
        return self.call_method('promoteUnchecked', {
            'blockId': block_id
        })
