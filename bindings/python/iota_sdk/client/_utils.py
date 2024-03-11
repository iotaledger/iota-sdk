# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from typing import Optional
from abc import ABCMeta, abstractmethod
from iota_sdk.types.address import Address
from iota_sdk.types.block.block import Block
from iota_sdk.types.block.id import BlockId
from iota_sdk.types.output import Output


class ClientUtils(metaclass=ABCMeta):
    """Client utility functions.
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

    def address_to_bech32(self, address: Address,
                          bech32_hrp: Optional[str] = None) -> str:
        """Converts an address to its bech32 representation.
        """
        return self._call_method('addressToBech32', {
            'address': address,
            'bech32Hrp': bech32_hrp
        })

    def computer_minimum_output_amount(self, output: Output) -> int:
        """Minimum required output amount.
        """
        return int(self._call_method(
            'computeMinimumOutputAmount', {
                'output': output.to_dict()
            }
        ))

    def request_funds_from_faucet(self, url: str, address: str) -> str:
        """Requests funds from the faucet, for example `https://faucet.testnet.shimmer.network/api/enqueue` or `http://localhost:8091/api/enqueue`.
        """
        return self._call_method(
            'requestFundsFromFaucet', {
                'url': url,
                'address': address,
            }
        )

    def block_id(self, block: Block) -> BlockId:
        """ Return a block ID (Blake2b256 hash of block bytes) from a block.
        """
        return BlockId(self._call_method('blockId', {
            'block': block,
        }))
