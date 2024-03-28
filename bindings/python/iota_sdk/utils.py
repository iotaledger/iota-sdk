# Copyright 2024 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from __future__ import annotations
import json
from typing import TYPE_CHECKING, List, Optional
from iota_sdk.common import custom_encoder
from iota_sdk.types.block.id import BlockId
from iota_sdk.types.signature import Ed25519Signature
from iota_sdk.types.address import Address, Ed25519Address, deserialize_address
from iota_sdk.types.common import HexStr
from iota_sdk.types.decayed_mana import DecayedMana
from iota_sdk.types.payload import Transaction, SignedTransactionPayload
from iota_sdk.types.node_info import ProtocolParameters, WorkScoreParameters
from iota_sdk.types.output import Output
from iota_sdk.types.output_id import OutputId
from iota_sdk.types.unlock import Unlock
from iota_sdk.types.transaction_data import InputSigningData
from iota_sdk.types.transaction_id import TransactionId
from iota_sdk.external import call_utils_method

# Required to prevent circular import
if TYPE_CHECKING:
    from iota_sdk.types.block.block import Block

# pylint: disable=too-many-public-methods


class Utils:
    """Utility functions.
    """
    @staticmethod
    def address_to_bech32(address: Address, bech32_hrp: str) -> str:
        """Convert an address to its bech32 representation.
        """
        return _call_method('addressToBech32', {
            'address': address,
            'bech32Hrp': bech32_hrp
        })

    # pylint: disable=redefined-builtin
    @staticmethod
    def public_key_hash(
            hex_str: HexStr) -> Ed25519Address:
        """Hashes a hex encoded public key with Blake2b256.
        """
        return Ed25519Address(_call_method('blake2b256Hash', {
            'bytes': hex_str,
        }))

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
        return _call_method('blake2b256Hash', {
            'bytes': output_id
        })

    @staticmethod
    def compute_delegation_id(output_id: OutputId) -> HexStr:
        """Compute the delegation id for the given account output id.
        """
        return _call_method('blake2b256Hash', {
            'bytes': output_id
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
    def compute_minimum_output_amount(output, storage_score_parameters) -> int:
        """Calculate the minimum required amount for an output.
        """
        return int(_call_method('computeMinimumOutputAmount', {
            'output': output,
            'storageScoreParameters': storage_score_parameters
        }))

    @staticmethod
    def compute_nft_id(output_id: OutputId) -> HexStr:
        """Compute the NFT id for the given NFT output id.
        """
        return _call_method('blake2b256Hash', {
            'bytes': output_id
        })

    @staticmethod
    def compute_output_id(transaction_id: TransactionId,
                          index: int) -> OutputId:
        """Compute the output id from transaction id and output index.
        """
        return OutputId(_call_method('computeOutputId', {
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
    def block_id(block: Block, params: ProtocolParameters) -> BlockId:
        """ Return a block ID (Blake2b256 hash of block bytes) from a block.
        """
        return BlockId(_call_method('blockId', {
            'block': block,
            'protocolParameters': params,
        }))

    @staticmethod
    def transaction_id(payload: SignedTransactionPayload) -> TransactionId:
        """ Compute the transaction ID (Blake2b256 hash of the provided transaction payload) of a transaction payload.
        """
        return TransactionId(_call_method('transactionId', {
            'payload': payload
        }))

    @staticmethod
    def protocol_parameters_hash(params: ProtocolParameters) -> HexStr:
        """ Compute the hash of a ProtocolParameters instance.
        """
        return _call_method('protocolParametersHash', {
            'protocolParameters': params,
        })

    @staticmethod
    def transaction_signing_hash(transaction: Transaction) -> HexStr:
        """ Compute the signing hash of a transaction.
        """
        return _call_method('transactionSigningHash', {
            'transaction': transaction,
        })

    @staticmethod
    def verify_ed25519_signature(
            signature: Ed25519Signature, message: HexStr) -> bool:
        """Verify an Ed25519 signature against a message.
        """
        return _call_method('verifyEd25519Signature', {
            'signature': signature,
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
            transaction: Transaction, inputs: List[InputSigningData], protocol_parameters: ProtocolParameters, unlocks: Optional[List[Unlock]] = None, mana_rewards: Optional[dict[OutputId, int]] = None):
        """Verifies the semantic of a transaction.
        """
        _call_method('verifyTransactionSemantic', {
            'transaction': transaction,
            'inputs': inputs,
            'unlocks': unlocks,
            'manaRewards': mana_rewards,
            'protocolParameters': protocol_parameters,
        })

    @staticmethod
    def mana_with_decay(
            mana: int, slot_index_created: int, slot_index_target: int, protocol_parameters: ProtocolParameters) -> int:
        """Applies mana decay to the given mana.
        """
        return int(_call_method('manaWithDecay', {
            'mana': str(mana),
            'slotIndexCreated': slot_index_created,
            'slotIndexTarget': slot_index_target,
            'protocolParameters': protocol_parameters,
        }))

    @staticmethod
    def generate_mana_with_decay(
            amount: int, slot_index_created: int, slot_index_target: int, protocol_parameters: ProtocolParameters) -> int:
        """Calculates the potential mana that is generated by holding `amount` tokens from `slot_index_created` to
        `slot_index_target` and applies the decay to the result.
        """
        return int(_call_method('generateManaWithDecay', {
            'amount': str(amount),
            'slotIndexCreated': slot_index_created,
            'slotIndexTarget': slot_index_target,
            'protocolParameters': protocol_parameters,
        }))

    @staticmethod
    def output_mana_with_decay(
            output: Output, slot_index_created: int, slot_index_target: int, protocol_parameters: ProtocolParameters) -> DecayedMana:
        """Calculates the potential mana that is generated by holding `amount` tokens from `slot_index_created` to
        `slot_index_target` and applies the decay to the result.
        """
        decayed_mana = _call_method('outputManaWithDecay', {
            'output': output,
            'slotIndexCreated': slot_index_created,
            'slotIndexTarget': slot_index_target,
            'protocolParameters': protocol_parameters,
        })

        return DecayedMana(int(decayed_mana["stored"]), int(
            decayed_mana["potential"]))

    @staticmethod
    def verify_transaction_syntax(
            transaction: SignedTransactionPayload, protocol_parameters: ProtocolParameters):
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
    def iota_mainnet_protocol_parameters() -> ProtocolParameters:
        """Returns sample protocol parameters for IOTA mainnet.
        """
        return ProtocolParameters.from_dict(
            _call_method('iotaMainnetProtocolParameters'))

    @staticmethod
    def shimmer_mainnet_protocol_parameters() -> ProtocolParameters:
        """Returns sample protocol parameters for Shimmer mainnet.
        """
        return ProtocolParameters.from_dict(
            _call_method('shimmerMainnetProtocolParameters'))

    @staticmethod
    def block_work_score(
            block: Block, work_score_parameters: WorkScoreParameters) -> int:
        """Returns the work score of a block.
        """
        return _call_method('blockWorkScore', {
            'block': block,
            'workScoreParameters': work_score_parameters,
        })


class UtilsError(Exception):
    """A utils error."""


def _call_utils_method_routine(func):
    """The routine of dump json string and call call_client_method().
    """
    def wrapper(*args, **kwargs):
        message = custom_encoder(func, *args, **kwargs)
        # Send message to the Rust library
        response = call_utils_method(message)

        json_response = json.loads(response)

        if "type" in json_response:
            if json_response["type"] == "error" or json_response["type"] == "panic":
                raise UtilsError(json_response['payload'])

        if "payload" in json_response:
            return json_response['payload']
        return response
    return wrapper


@_call_utils_method_routine
def _call_method(name, data=None):
    """Dumps json string and calls `call_client_method()`
    """
    message = {
        'name': name
    }
    if data:
        message['data'] = data
    return message
