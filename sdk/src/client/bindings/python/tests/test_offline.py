
# Copyright 2022 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from iota_client import IotaClient, MnemonicSecretManager, OutputId, hex_to_utf8, utf8_to_hex
import json
import unittest

# Read the test vector
tv = dict()
with open('../../../../tests/client/fixtures/test_vectors.json') as json_file:
    tv = json.load(json_file)

client = IotaClient()


def test_mnemonic_address_generation():
    mnemonic_address_test_cases = tv['general']['address_generations']

    for test in mnemonic_address_test_cases:
        secret_manager = MnemonicSecretManager(test['mnemonic'])

        generated_address = client.generate_addresses(secret_manager,
                                                      coin_type=test['coin_type'],
                                                      account_index=test['account_index'],
                                                      bech32_hrp=test['bech32_hrp'],
                                                      internal=test['internal'],
                                                      start=test['address_index'],
                                                      end=test['address_index']+1
                                                      )

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


class TestTypes(unittest.TestCase):
    def test_output_id(self):
        transaction_id = '0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c649'
        output_index = 42
        output_id = OutputId(transaction_id, output_index)
        assert output_id.__repr__(
        ) == '0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c6492a00'

        new_output_id = OutputId.from_string(
            '0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c6492a00')
        assert new_output_id.__repr__(
        ) == '0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c6492a00'
        assert new_output_id.transaction_id == transaction_id
        assert new_output_id.output_index == output_index

        transaction_id_missing_0x_prefix = '52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c649'
        with self.assertRaises(ValueError):
            OutputId(transaction_id_missing_0x_prefix, output_index)
        transaction_id__invalid_hex_prefix = '0052fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c649'
        with self.assertRaises(ValueError):
            OutputId(transaction_id__invalid_hex_prefix, output_index)
        transaction_id_invalid_hex_char = '0xz2fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c649'
        with self.assertRaises(ValueError):
            OutputId(transaction_id_invalid_hex_char, output_index)
        invalid_output_index = 129
        with self.assertRaises(ValueError):
            OutputId(transaction_id, invalid_output_index)
        output_id_missing_0x_prefix = '52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c6492a00'
        with self.assertRaises(ValueError):
            OutputId.from_string(output_id_missing_0x_prefix)
        output_id_invalid_hex_char = '0xz2fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c6492a00'
        with self.assertRaises(ValueError):
            OutputId.from_string(output_id_invalid_hex_char)
        output_id_invalid_hex_prefix = '0052fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c6492a00'
        with self.assertRaises(ValueError):
            OutputId.from_string(output_id_invalid_hex_prefix)


def test_hex_utf8():
    utf8_data = "Don't panic!"
    hex_data = '0x446f6e27742070616e696321'
    assert utf8_to_hex(utf8_data) == hex_data
    assert hex_to_utf8(hex_data) == utf8_data
