
# Copyright 2022 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from iota_client import IotaClient, MnemonicSecretManager
import json

# Read the test vector
tv = dict()
with open('../../tests/fixtures/test_vectors.json') as json_file:
    tv = json.load(json_file)

client = IotaClient()


def test_mnemonic_address_generation():
    mnemonic_address_test_cases = tv['general']['address_generations']

    for test in mnemonic_address_test_cases:
        secret_manager = MnemonicSecretManager(test['mnemonic'])

        generated_address = client.generate_addresses(secret_manager, {
            "coinType": test['coin_type'],
            "accountIndex": test['account_index'],
            "bech32Hrp": test['bech32_hrp'],
            "internal": test['internal'],
            "range": {
                "start": test['address_index'],
                "end": test['address_index']+1,
            },
        })

        assert test['bech32_address'] == generated_address[0]

def test_sign_verify_ed25519():
    secret_manager = MnemonicSecretManager(client.generate_mnemonic())
    # `IOTA` hex encoded
    message = '0x494f5441'

    signature = client.sign_ed25519(
        secret_manager,
        message,
        # [44, 4218, 0, 0, 0] IOTA coin type, first account, first public address
        [
            {'hardened': True, 'bs': [128, 0, 0, 44]},
            {'hardened': True, 'bs': [128, 0, 16, 123]},
            {'hardened': True, 'bs': [128, 0, 0, 0]},
            {'hardened': True, 'bs': [128, 0, 0, 0]},
            {'hardened': True, 'bs': [128, 0, 0, 0]},
        ],
    )

    bech32_address = client.hex_public_key_to_bech32_address(
        signature['publicKey'],
        'rms',
    )

    pub_key_hash = client.bech32_to_hex(bech32_address)

    valid_signature = client.verify_ed25519_signature(
        signature,
        message,
        {'type': 0, 'pubKeyHash': pub_key_hash},
    )
    assert valid_signature
