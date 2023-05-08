# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from iota_sdk import call_utils_method
from json import dumps, loads


class Utils():

    @staticmethod
    def bech32_to_hex(bech32):
        """Transforms bech32 to hex.
        """
        return _call_method('bech32ToHex', {
            'bech32': bech32
        })

    @staticmethod
    def hex_to_bech32(hex, bech32_hrp):
        """Transforms a hex encoded address to a bech32 encoded address.
        """
        return _call_method('hexToBech32', {
            'hex': hex,
            'bech32Hrp': bech32_hrp
        })

    @staticmethod
    def alias_id_to_bech32(alias_id, bech32_hrp):
        """Transforms an alias id to a bech32 encoded address.
        """
        return _call_method('aliasIdToBech32', {
            'aliasId': alias_id,
            'bech32Hrp': bech32_hrp
        })

    @staticmethod
    def nft_id_to_bech32(nft_id, bech32_hrp):
        """Transforms an nft id to a bech32 encoded address.
        """
        return _call_method('nftIdToBech32', {
            'nftId': nft_id,
            'bech32Hrp': bech32_hrp
        })

    @staticmethod
    def hex_public_key_to_bech32_address(hex, bech32_hrp=None):
        """Transforms a hex encoded public key to a bech32 encoded address.
        """
        return _call_method('hexPublicKeyToBech32Address', {
            'hex': hex,
            'bech32Hrp': bech32_hrp
        })

    @staticmethod
    def parse_bech32_address(address):
        """Returns a valid Address parsed from a String.
        """
        return _call_method('parseBech32Address', {
            'address': address
        })

    @staticmethod
    def is_address_valid(address):
        """Checks if a String is a valid bech32 encoded address.
        """
        return _call_method('isAddressValid', {
            'address': address
        })

    @staticmethod
    def generate_mnemonic():
        """Generates a new mnemonic.
        """
        return _call_method('generateMnemonic')

    @staticmethod
    def mnemonic_to_hex_seed(mnemonic):
        """Returns a hex encoded seed for a mnemonic.
        """
        return _call_method('mnemonicToHexSeed', {
            'mnemonic': mnemonic
        })

    @staticmethod
    def compute_alias_id(output_id):
        """Computes the alias id for the given alias output id.
        """
        return _call_method('computeAliasId', {
            'outputId': output_id
        })

    @staticmethod
    def compute_nft_id(output_id):
        """Computes the NFT id for the given NFT output id.
        """
        return _call_method('computeNftId', {
            'outputId': output_id
        })

    @staticmethod
    def compute_foundry_id(alias_address, serial_number, token_scheme_kind):
        """Computes the foundry id.
        """
        return _call_method('computeNftId', {
            'aliasAddress': alias_address,
            'serialNumber': serial_number,
            'tokenSchemeKind': token_scheme_kind
        })

    @staticmethod
    def block_id(block):
        """ Returns a block ID (Blake2b256 hash of block bytes) from a block.
        """
        return _call_method('blockId', {
            'block': block
        })

    @staticmethod
    def hash_transaction_essence(essence):
        """ Compute the hash of a transaction essence.
        """
        return _call_method('hashTransactionEssence', {
            'essence': essence
        })

    @staticmethod
    def verify_ed25519_signature(signature, message, address):
        """Verifies the Ed25519Signature for a message against an Ed25519Address.
        """
        return _call_method('verifyEd25519Signature', {
            'signature': signature,
            'message': message,
            'address': address,
        })


class UtilsError(Exception):
    """utils error"""
    pass


def _call_method(name, data=None):
    """Dumps json string and call call_utils_method()
    """
    message = {
        'name': name
    }
    if data:
        message['data'] = data
    message = dumps(message)

    # Send message to the Rust library
    response = call_utils_method(message)

    json_response = loads(response)

    if "type" in json_response:
        if json_response["type"] == "error":
            raise UtilsError(json_response['payload'])

    if "payload" in json_response:
        return json_response['payload']
    else:
        return response
