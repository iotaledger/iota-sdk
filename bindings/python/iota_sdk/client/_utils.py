# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from typing import Optional
from abc import ABCMeta, abstractmethod

from iota_sdk.types.common import HexStr
from iota_sdk.types.output import Output


class ClientUtils(metaclass=ABCMeta):
    """Client utility functions.
    """

    @abstractmethod
    def _call_method(self, name, data=None):
        return {}

    def hex_to_bech32(self, hex_str: HexStr, bech32_hrp: str) -> str:
        """Transforms a hex encoded address to a bech32 encoded address.
        """
        return self._call_method('hexToBech32', {
            'hex': hex_str,
            'bech32Hrp': bech32_hrp
        })

    def account_id_to_bech32(self, account_id: HexStr, bech32_hrp: str) -> str:
        """Transforms an account id to a bech32 encoded address.
        """
        return self._call_method('accountIdToBech32', {
            'accountId': account_id,
            'bech32Hrp': bech32_hrp
        })

    def nft_id_to_bech32(self, nft_id: HexStr, bech32_hrp: str) -> str:
        """Transforms an nft id to a bech32 encoded address.
        """
        return self._call_method('nftIdToBech32', {
            'nftId': nft_id,
            'bech32Hrp': bech32_hrp
        })

    def hex_public_key_to_bech32_address(
            self, hex_str: HexStr, bech32_hrp: Optional[str] = None) -> str:
        """Transforms a hex encoded public key to a bech32 encoded address.
        """
        return self._call_method('hexPublicKeyToBech32Address', {
            'hex': hex_str,
            'bech32Hrp': bech32_hrp
        })

    def minimum_required_storage_deposit(self, output: Output) -> int:
        """Minimum required storage deposit.
        """
        return int(self._call_method(
            'minimumRequiredStorageDeposit', {
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
