# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from json import dumps, loads
from typing import Optional, Union
import humps
from iota_sdk.external import create_secret_manager, call_secret_manager_method
from iota_sdk.types.block.block import Block, UnsignedBlock
from iota_sdk.types.common import HexStr
from iota_sdk.types.node_info import ProtocolParameters
from iota_sdk.types.payload import SignedTransactionPayload
from iota_sdk.types.signature import Ed25519Signature, Bip44
from iota_sdk.types.transaction_data import PreparedTransactionData


class LedgerNanoSecretManager(dict):
    """Secret manager that uses a Ledger Nano hardware wallet or a Speculos simulator.
    """

    def __init__(self, is_simulator):
        """Initialize a Ledger Nano secret manager.

        Args:
            is_simulator: Whether this is a simulated Ledger Nano device.
        """

        dict.__init__(self, ledgerNano=is_simulator)


class MnemonicSecretManager(dict):
    """Secret manager that uses a mnemonic held in memory.
    This is not recommended in production. Use LedgerNano or Stronghold instead.
    """

    def __init__(self, mnemonic):
        """Initialize a mnemonic secret manager.

        Args:
            mnemonic: The root secret of this type of secret manager.
        """

        dict.__init__(self, mnemonic=mnemonic)


class SeedSecretManager(dict):
    """Secret manager that uses a seed.
    """

    def __init__(self, seed):
        """Initialize a seed secret manager.

        Args:
            seed: The root secret of this type of secret manager.
        """

        dict.__init__(self, hexSeed=seed)


class StrongholdSecretManager(dict):
    """Secret manager that uses Stronghold.
    """

    def __init__(self, snapshot_path, password):
        """Initialize a stronghold secret manager.

        Args:
            snapshot_path: The path to the Stronghold snapshot file.
            password: The password to unlock the Stronghold snapshot file.
        """

        dict.__init__(self, stronghold=StrongholdSecretManager.Inner(
            snapshot_path, password))

    class Inner(dict):
        """Inner storage for stronghold configuration information.
        """

        def __init__(self, snapshot_path, password):
            dict.__init__(self, password=password, snapshotPath=snapshot_path)


class SecretManagerError(Exception):
    """Secret manager error.
    """


class SecretManager:
    """Secret manager wrapper.
    """

    def __init__(self, secret_manager: Optional[Union[LedgerNanoSecretManager, MnemonicSecretManager,
                 SeedSecretManager, StrongholdSecretManager]] = None, secret_manager_handle=None):
        """Initialize a secret manager.

        Args:
            secret_manager: One of the supported secret managers.
            secret_manager_handle: A handle to a secret manager.
        """

        if secret_manager_handle is None:
            self.handle = create_secret_manager(dumps(secret_manager))
        else:
            self.handle = secret_manager_handle

    def _call_method(self, name, data=None):
        """Dumps json string and call call_secret_manager_method()
        """
        message = {
            'name': name
        }
        if data:
            message['data'] = data
        message = dumps(message)

        # Send message to the Rust library
        response = call_secret_manager_method(self.handle, message)

        json_response = loads(response)

        if "type" in json_response:
            if json_response["type"] == "error":
                raise SecretManagerError(json_response['payload'])

        if "payload" in json_response:
            return json_response['payload']
        return response

    # TODO: Should we include `bech32` in the method name?
    def generate_ed25519_address(self,
                                 coin_type: int,
                                 bech32_hrp: str,
                                 account_index: Optional[int] = None,
                                 address_index: Optional[int] = None,
                                 internal: Optional[bool] = None,
                                 legder_nano_prompt: Optional[bool] = None):
        """Generate a single Ed25519 address.

        Args:
            coin_type: The coin type to generate the address for.
            bech32_hrp: The bech32 HRP (human readable part) to use.
            account_index: An account index.
            address_index: An address index.
            internal: Whether the generated address should be internal.
            ledger_nano_prompt: Whether to display the address on Ledger Nano devices.

        Returns:
            The generated Ed25519 address.
        """

        options = {}
        options['coinType'] = coin_type
        options['bech32Hrp'] = bech32_hrp
        if address_index is not None:
            options['range'] = {}
            options['range']['start'] = address_index
            options['range']['end'] = address_index + 1
        if account_index is not None:
            options['accountIndex'] = account_index
        if internal is not None or legder_nano_prompt is not None:
            options['options'] = {}
            if internal is not None:
                options['options']['internal'] = internal
            if legder_nano_prompt is not None:
                options['options']['ledgerNanoPrompot'] = legder_nano_prompt

        return self._call_method('generateEd25519Addresses', {
            'options': options
        })[0]

    # pylint: disable=unused-argument

    # TODO: Should `coin_type` and `bech32_hrp` be mandatory to provide?
    # TODO: Should we include `bech32` in the method name?
    def generate_ed25519_addresses(self,
                                   coin_type: Optional[int] = None,
                                   bech32_hrp: Optional[str] = None,
                                   account_index: Optional[int] = None,
                                   start: Optional[int] = None,
                                   end: Optional[int] = None,
                                   internal: Optional[bool] = None,
                                   ledger_nano_prompt: Optional[bool] = None):
        """Generate multiple Ed25519 addresses at once.

        Args:
            coin_type: The coin type to generate addresses for.
            bech32_hrp: The bech32 HRP (human readable part) to use.
            account_index: An account index.
            start: The start index of the addresses to generate.
            end: The end index of the addresses to generate.
            internal: Whether the generated addresses should be internal.
            ledger_nano_prompt: Whether to display the address on Ledger Nano devices.

        Returns:
            The generated Ed25519 addresses.
        """
        options = dict(locals())
        del options['self']

        options = {k: v for k, v in options.items() if v is not None}

        is_start_set = 'start' in options
        is_end_set = 'end' in options
        if is_start_set or is_end_set:
            options['range'] = {}
            if is_start_set:
                options['range']['start'] = options.pop('start')
            if is_end_set:
                options['range']['end'] = options.pop('end')
        if 'coin_type' in options:
            options['coin_type'] = int(options.pop('coin_type'))
        if 'internal' in options:
            if 'options' not in options:
                options['options'] = {}
            options['options']['internal'] = options.pop('internal')
        if 'ledger_nano_prompt' in options:
            if 'options' not in options:
                options['options'] = {}
            options['options']['ledger_nano_prompt'] = options.pop(
                'ledger_nano_prompt')

        options = humps.camelize(options)

        return self._call_method('generateEd25519Addresses', {
            'options': options
        })

    # pylint: disable=unused-argument
    def generate_evm_addresses(self,
                               account_index: Optional[int] = None,
                               start: Optional[int] = None,
                               end: Optional[int] = None,
                               internal: Optional[bool] = None,
                               coin_type: Optional[int] = None,
                               ledger_nano_prompt: Optional[bool] = None):
        """Generate EVM addresses.

        Args:
            account_index: An account index.
            start: The start index of the addresses to generate.
            end: The end index of the addresses to generate.
            internal: Whether the generated addresses should be internal.
            coin_type: The coin type to generate addresses for.
            ledger_nano_prompt: Whether to display the address on Ledger Nano devices.

        Returns:
            The generated EVM addresses.
        """
        options = dict(locals())
        del options['self']

        options = {k: v for k, v in options.items() if v is not None}

        is_start_set = 'start' in options
        is_end_set = 'end' in options
        if is_start_set or is_end_set:
            options['range'] = {}
            if is_start_set:
                options['range']['start'] = options.pop('start')
            if is_end_set:
                options['range']['end'] = options.pop('end')
        if 'coin_type' in options:
            options['coin_type'] = int(options.pop('coin_type'))
        if 'ledger_nano_prompt' in options:
            options['options'] = {
                'ledger_nano_prompt': options.pop('ledger_nano_prompt')}

        options = humps.camelize(options)

        return self._call_method('generateEvmAddresses', {
            'options': options
        })

    def get_ledger_nano_status(self):
        """Return the Ledger Status.
        """
        return self._call_method('getLedgerNanoStatus')

    def store_mnemonic(self, mnemonic: str):
        """Store a mnemonic.

        Args:
            mnemonic: A mnemonic to store in the secret manager.
        """
        return self._call_method('storeMnemonic', {
            'mnemonic': mnemonic
        })

    def sign_ed25519(self, message: HexStr, chain: Bip44) -> Ed25519Signature:
        """Signs a message with an Ed25519 private key.

        Args:
            message: The given message to sign.
            chain: The chain to sign with.

        Returns:
            The Ed25519 signature.
        """
        return Ed25519Signature.from_dict(self._call_method('signEd25519', {
            'message': message,
            'chain': chain.to_dict(),
        }))

    def sign_secp256k1_ecdsa(self, message: HexStr, chain: Bip44):
        """Signs a message with an Secp256k1Ecdsa private key.

        Args:
            message: The given message to sign.
            chain: The chain to sign with.
        """
        return self._call_method('signSecp256k1Ecdsa', {
            'message': message,
            'chain': chain.to_dict(),
        })

    def sign_transaction(
            self, prepared_transaction_data: PreparedTransactionData, protocol_parameters: ProtocolParameters) -> SignedTransactionPayload:
        """Sign a transaction.

        Args:
            prepare_transaction_data: The prepared transaction data that needs to be signed.
            protocol_parameters: The protocol parameters used in creating the signed transaction.
        """
        return SignedTransactionPayload.from_dict(self._call_method('signTransaction', {
            'preparedTransactionData': prepared_transaction_data.to_dict(),
            'protocolParameters': protocol_parameters
        }))

    def sign_block(
            self, unsigned_block: UnsignedBlock, chain: Bip44) -> Block:
        """Sign a block.

        Args:
            unsigned_block: The unsigned block data.
            chain: The Bip44 chain to use.
        """
        return Block.from_dict(self._call_method('signBlock', {
            'unsignedBlock': unsigned_block.to_dict(),
            'chain': chain.to_dict()
        }))

    def signature_unlock(self, transaction_signing_hash: HexStr, chain: Bip44):
        """Sign a transaction hash.

        Args:
            transaction_signing_hash: The transaction hash to sign.
            chain: The chain to sign with.
        """
        return self._call_method('signatureUnlock', {
            'transactionSigningHash': transaction_signing_hash,
            'chain': chain.to_dict(),
        })
