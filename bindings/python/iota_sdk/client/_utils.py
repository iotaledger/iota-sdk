# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

class ClientUtils():

    def hex_to_bech32(self, hex, bech32_hrp):
        """Transforms a hex encoded address to a bech32 encoded address.
        """
        return self._call_method('hexToBech32', {
            'hex': hex,
            'bech32Hrp': bech32_hrp
        })

    def alias_id_to_bech32(self, alias_id, bech32_hrp):
        """Transforms an alias id to a bech32 encoded address.
        """
        return self._call_method('aliasIdToBech32', {
            'aliasId': alias_id,
            'bech32Hrp': bech32_hrp
        })

    def nft_id_to_bech32(self, nft_id, bech32_hrp):
        """Transforms an nft id to a bech32 encoded address.
        """
        return self._call_method('nftIdToBech32', {
            'nftId': nft_id,
            'bech32Hrp': bech32_hrp
        })

    def hex_public_key_to_bech32_address(self, hex, bech32_hrp=None):
        """Transforms a hex encoded public key to a bech32 encoded address.
        """
        return self._call_method('hexPublicKeyToBech32Address', {
            'hex': hex,
            'bech32Hrp': bech32_hrp
        })

    def request_funds_from_faucet(self, url, address):
        """Requests funds from the faucet, for example `https://faucet.testnet.shimmer.network/api/enqueue` or `http://localhost:8091/api/enqueue`.
        """
        return self._call_method(
            'requestFundsFromFaucet', {
                'url': url,
                'address': address,
            }
        )
