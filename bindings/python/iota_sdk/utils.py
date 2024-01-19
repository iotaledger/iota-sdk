# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
from json import dumps, loads
from typing import TYPE_CHECKING, List
from dacite import from_dict

from iota_sdk.types.signature import Ed25519Signature
from iota_sdk.types.address import Address, AddressType, Ed25519Address, AliasAddress, NFTAddress
from iota_sdk.types.common import HexStr
from iota_sdk.types.output_id import OutputId
from iota_sdk.types.output import Output
from iota_sdk.types.transaction_data import InputSigningData
from iota_sdk.external import call_utils_method
from iota_sdk.types.node_info import NodeInfoProtocol
from iota_sdk.types.payload import TransactionPayload

# Required to prevent circular import
if TYPE_CHECKING:
    from iota_sdk.types.block import Block


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
    def hex_to_bech32(hex: HexStr, bech32_hrp: str) -> str:
        """Convert a hex encoded address to a Bech32 encoded address.
        """
        return _call_method('hexToBech32', {
            'hex': hex,
            'bech32Hrp': bech32_hrp
        })

    @staticmethod
    def alias_id_to_bech32(alias_id: HexStr, bech32_hrp: str) -> str:
        """Convert an alias id to a Bech32 encoded address.
        """
        return _call_method('aliasIdToBech32', {
            'aliasId': alias_id,
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
    def hex_public_key_to_bech32_address(hex: HexStr, bech32_hrp: str) -> str:
        """Convert a hex encoded public key to a Bech32 encoded address.
        """
        return _call_method('hexPublicKeyToBech32Address', {
            'hex': hex,
            'bech32Hrp': bech32_hrp
        })

    @staticmethod
    def parse_bech32_address(address: str) -> Address:
        """Parse a string into a valid address.
        """
        response = _call_method('parseBech32Address', {
            'address': address
        })

        address_type = AddressType(response['type'])

        if address_type == AddressType.ED25519:
            return from_dict(Ed25519Address, response)
        if address_type == AddressType.ALIAS:
            return from_dict(AliasAddress, response)
        if address_type == AddressType.NFT:
            return from_dict(NFTAddress, response)
        return from_dict(Address, response)

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
    def compute_alias_id(output_id: OutputId) -> HexStr:
        """Compute the alias id for the given alias output id.
        """
        return _call_method('computeAliasId', {
            'outputId': repr(output_id)
        })

    @staticmethod
    def compute_foundry_id(alias_id: HexStr, serial_number: int,
                           token_scheme_type: int) -> HexStr:
        """Compute the foundry id.
        """
        return _call_method('computeFoundryId', {
            'aliasId': alias_id,
            'serialNumber': serial_number,
            'tokenSchemeType': token_scheme_type
        })

    @staticmethod
    def compute_inputs_commitment(inputs: List[Output]) -> HexStr:
        """Compute the input commitment from the output objects that are used as inputs to fund the transaction.
        """
        return _call_method('computeInputsCommitment', {
            'inputs': [i.as_dict() for i in inputs]
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
    def compute_token_id(alias_id: HexStr, serial_number: int,
                         token_scheme_type: int) -> HexStr:
        """Compute a token id from the alias id, serial number and token scheme type.
        """
        return _call_method('computeTokenId', {
            'aliasId': alias_id,
            'serialNumber': serial_number,
            'tokenSchemeType': token_scheme_type
        })

    @staticmethod
    def block_id(block: Block) -> HexStr:
        """ Return a block ID (Blake2b256 hash of block bytes) from a block.
        """
        return _call_method('blockId', {
            'block': block.as_dict()
        })

    @staticmethod
    def transaction_id(transaction_payload: TransactionPayload) -> HexStr:
        """ Compute the transaction ID (Blake2b256 hash of the provided transaction payload) of a transaction payload.
        """
        return _call_method('transactionId', {
            'payload': transaction_payload.as_dict()
        })

    @staticmethod
    def hash_transaction_essence(essence) -> HexStr:
        """ Compute the hash of a transaction essence.
        """
        return _call_method('hashTransactionEssence', {
            'essence': essence
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
            inputs: List[InputSigningData], transaction: TransactionPayload, time: int) -> str:
        """Verifies the semantic of a transaction.
        """
        return _call_method('verifyTransactionSemantic', {
            'inputs': [i.as_dict() for i in inputs],
            'transaction': transaction.as_dict(),
            'time': time,
        })

    @staticmethod
    def verify_transaction_syntax(
            transaction: TransactionPayload, protocol_parameters: NodeInfoProtocol):
        """Verifies the syntax of a transaction.
        """
        _call_method('verifyTransactionSyntax', {
            'transaction': transaction.as_dict(),
            'protocolParameters': protocol_parameters.as_dict(),
        })

    @staticmethod
    def block_bytes(
            block: Block) -> bytes:
        """Returns the serialized bytes of a block.
        """
        return bytes(_call_method('blockBytes', {
            'block': block.as_dict(),
        }))

    @staticmethod
    def block_hash_without_nonce(
            block: Block) -> HexStr:
        """Returns a block hash (Blake2b256 hash of block bytes without nonce) from a block for PoW.
        """
        return _call_method('blockHashWithoutNonce', {
            'block': block.as_dict(),
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
