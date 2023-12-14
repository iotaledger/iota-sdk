# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from json import dumps, loads
from typing import TYPE_CHECKING, List, Optional

from iota_sdk.types.signature import Ed25519Signature
from iota_sdk.types.address import Address, deserialize_address
from iota_sdk.types.common import HexStr
from iota_sdk.types.transaction import Transaction
from iota_sdk.types.node_info import ProtocolParameters
from iota_sdk.types.output_id import OutputId
from iota_sdk.types.unlock import Unlock
from iota_sdk.external import call_utils_method
from iota_sdk.types.payload import SignedTransactionPayload
from iota_sdk.types.transaction_data import InputSigningData

# Required to prevent circular import
if TYPE_CHECKING:
    from iota_sdk.types.block.block import Block

# pylint: disable=too-many-public-methods


class Utils():
    """Utility functions.
    """

    @staticmethod
    def bech32_to_hex(bech32: str) -> HexStr:
        """Convert a Bech32 string to a hex string.
        """
        return _call_method('bech32ToHex', {
            'bech32': bech32
        })

    # pylint: disable=redefined-builtin
    @staticmethod
    def hex_to_bech32(hex_str: HexStr, bech32_hrp: str) -> str:
        """Convert a hex encoded address to a Bech32 encoded address.
        """
        return _call_method('hexToBech32', {
            'hex': hex_str,
            'bech32Hrp': bech32_hrp
        })

    @staticmethod
    def account_id_to_bech32(account_id: HexStr, bech32_hrp: str) -> str:
        """Convert an account id to a Bech32 encoded address.
        """
        return _call_method('accountIdToBech32', {
            'accountId': account_id,
            'bech32Hrp': bech32_hrp
        })

    @staticmethod
    def nft_id_to_bech32(nft_id: HexStr, bech32_hrp: str) -> str:
        """Convert an NFT ID to a Bech32 encoded address.
        """
        return _call_method('nftIdToBech32', {
            'nftId': nft_id,
            'bech32Hrp': bech32_hrp
        })

    # pylint: disable=redefined-builtin
    @staticmethod
    def hex_public_key_to_bech32_address(
            hex_str: HexStr, bech32_hrp: str) -> str:
        """Convert a hex encoded public key to a Bech32 encoded address.
        """
        return _call_method('hexPublicKeyToBech32Address', {
            'hex': hex_str,
            'bech32Hrp': bech32_hrp
        })

    @staticmethod
    def parse_bech32_address(address: str) -> Address:
        """Parse a string into a valid address.
        """
        response = _call_method('parseBech32Address', {
            'address': address
        })

        return deserialize_address(response)

    @staticmethod
    def is_address_valid(address: str) -> bool:
        """Check whether a string is a valid Bech32 encoded address.
        """
        return _call_method('isAddressValid', {
            'address': address
        })

    @staticmethod
    def generate_mnemonic() -> str:
        """Generate a new mnemonic.
        """
        return _call_method('generateMnemonic')

    @staticmethod
    def mnemonic_to_hex_seed(mnemonic: str) -> HexStr:
        """Convert a mnemonic to a hex encoded seed.
        """
        return _call_method('mnemonicToHexSeed', {
            'mnemonic': mnemonic
        })

    @staticmethod
    def compute_account_id(output_id: OutputId) -> HexStr:
        """Compute the account id for the given account output id.
        """
        return _call_method('computeAccountId', {
            'outputId': repr(output_id)
        })

    @staticmethod
    def compute_foundry_id(account_id: HexStr, serial_number: int,
                           token_scheme_type: int) -> HexStr:
        """Compute the foundry id.
        """
        return _call_method('computeFoundryId', {
            'accountId': account_id,
            'serialNumber': serial_number,
            'tokenSchemeType': token_scheme_type
        })

    @staticmethod
    def compute_storage_deposit(output, rent) -> HexStr:
        """Compute the required storage deposit of an output.
        """
        return _call_method('computeStorageDeposit', {
            'output': output,
            'rent': rent
        })

    @staticmethod
    def compute_nft_id(output_id: OutputId) -> HexStr:
        """Compute the NFT id for the given NFT output id.
        """
        return _call_method('computeNftId', {
            'outputId': repr(output_id)
        })

    @staticmethod
    def compute_output_id(transaction_id: HexStr, index: int) -> OutputId:
        """Compute the output id from transaction id and output index.
        """
        return OutputId.from_string(_call_method('computeOutputId', {
            'id': transaction_id,
            'index': index,
        }))

    @staticmethod
    def compute_token_id(account_id: HexStr, serial_number: int,
                         token_scheme_type: int) -> HexStr:
        """Compute a token id from the account id, serial number and token scheme type.
        """
        return _call_method('computeTokenId', {
            'accountId': account_id,
            'serialNumber': serial_number,
            'tokenSchemeType': token_scheme_type
        })

    @staticmethod
    def block_id(block: Block, params: ProtocolParameters) -> HexStr:
        """ Return a block ID (Blake2b256 hash of block bytes) from a block.
        """
        return _call_method('blockId', {
            'block': block.to_dict(),
            'protocol_parameters': params.to_dict(),
        })

    @staticmethod
    def transaction_id(payload: SignedTransactionPayload) -> HexStr:
        """ Compute the transaction ID (Blake2b256 hash of the provided transaction payload) of a transaction payload.
        """
        return _call_method('transactionId', {
            'payload': payload.as_dict()
        })

    @staticmethod
    def protocol_parameters_hash(params: ProtocolParameters) -> HexStr:
        """ Compute the hash of a ProtocolParameters instance.
        """
        return _call_method('protocolParametersHash', {
            'protocolParameters': params.to_dict(),
        })

    @staticmethod
    def transaction_signing_hash(transaction: Transaction) -> HexStr:
        """ Compute the signing hash of a transaction.
        """
        return _call_method('transactionSigningHash', {
            'transaction': transaction.to_dict(),
        })

    @staticmethod
    def verify_ed25519_signature(
            signature: Ed25519Signature, message: HexStr) -> bool:
        """Verify an Ed25519 signature against a message.
        """
        return _call_method('verifyEd25519Signature', {
            'signature': signature.__dict__,
            'message': message,
        })

    @staticmethod
    def verify_secp256k1_ecdsa_signature(
            public_key: HexStr, signature: HexStr, message: HexStr) -> bool:
        """Verify a Secp256k1Ecdsa signature against a message.
        """
        return _call_method('verifySecp256k1EcdsaSignature', {
            'publicKey': public_key,
            'signature': signature,
            'message': message,
        })

    @staticmethod
    def verify_transaction_semantic(
            transaction: Transaction, inputs: List[InputSigningData], unlocks: Optional[List[Unlock]] = None) -> str:
        """Verifies the semantic of a transaction.
        """
        return _call_method('verifyTransactionSemantic', {
            'transaction': transaction.as_dict(),
            'inputs': [i.as_dict() for i in inputs],
            'unlocks': [u.as_dict() for u in unlocks],
        })


class UtilsError(Exception):
    """A utils error."""


def _call_method(name: str, data=None):
    """Dumps json string and call call_utils_method().
    """
    message = {
        'name': name
    }
    if data:
        message['data'] = data
    message_str: str = dumps(message)

    # Send message to the Rust library
    response = call_utils_method(message_str)

    json_response = loads(response)

    if "type" in json_response:
        if json_response["type"] == "error":
            raise UtilsError(json_response['payload'])

    if "payload" in json_response:
        return json_response['payload']

    return response
