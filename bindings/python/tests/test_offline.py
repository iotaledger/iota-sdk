# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

import json
import unittest
from iota_sdk import Client, MnemonicSecretManager, Utils, SecretManager, OutputId, hex_to_utf8, utf8_to_hex, Bip44, CoinType, Irc27Metadata, Irc30Metadata, TransactionId


# Read the test vector
tv = {}
with open('../../sdk/tests/client/fixtures/test_vectors.json', "r", encoding="utf-8") as json_file:
    tv = json.load(json_file)

client = Client(protocol_parameters=Utils.iota_mainnet_protocol_parameters())


def test_mnemonic_address_generation():
    mnemonic_address_test_cases = tv['general']['address_generations']

    for test in mnemonic_address_test_cases:
        secret_manager = SecretManager(MnemonicSecretManager(test['mnemonic']))

        generated_address = secret_manager.generate_ed25519_addresses(
            coin_type=test['coin_type'],
            account_index=test['account_index'],
            bech32_hrp=test['bech32_hrp'],
            internal=test['internal'],
            start=test['address_index'],
            end=test['address_index'] + 1
        )

        assert test['bech32_address'] == generated_address[0]


def test_sign_verify_ed25519():
    secret_manager = MnemonicSecretManager(
        "acoustic trophy damage hint search taste love bicycle foster cradle brown govern endless depend situate athlete pudding blame question genius transfer van random vast")
    message = utf8_to_hex('IOTA')

    bip_path = Bip44(
        coin_type=CoinType.IOTA
    )

    secret_manager = SecretManager(secret_manager)
    signature = secret_manager.sign_ed25519(
        message,
        bip_path,
    )
    assert signature.signature == '0x72bf2bc8fbc5dc56d657d7de8afa5208be1db025851e81031c754b371c7a29ce9f352d12df8207f9163316f81f59eb7725e5c0e4f3228e71ffe3764a9de6b10e'

    valid_signature = Utils.verify_ed25519_signature(
        signature,
        message,
    )
    assert valid_signature


class TestTypes(unittest.TestCase):
    def test_output_id(self):
        transaction_id = TransactionId(
            '0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c64900000000')
        output_index = 42
        output_id = OutputId.from_transaction_id_and_output_index(
            transaction_id, output_index)
        assert str(output_id
                   ) == "0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c649000000002a00"

        new_output_id = OutputId(
            '0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c649000000002a00')
        assert str(new_output_id
                   ) == '0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c649000000002a00'
        assert new_output_id.transaction_id() == transaction_id
        assert new_output_id.output_index() == output_index

        output_id_invalid_hex_char = '0xz2fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c649000000002a00'
        with self.assertRaises(ValueError):
            OutputId(output_id_invalid_hex_char)

        transaction_id_missing_0x_prefix = '52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c64900000000'
        with self.assertRaises(ValueError):
            OutputId.from_transaction_id_and_output_index(
                transaction_id_missing_0x_prefix, output_index)
        transaction_id__invalid_hex_prefix = '0052fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c64900000000'
        with self.assertRaises(ValueError):
            OutputId.from_transaction_id_and_output_index(
                transaction_id__invalid_hex_prefix, output_index)
        transaction_id_invalid_hex_char = '0xz2fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c64900000000'
        with self.assertRaises(ValueError):
            OutputId.from_transaction_id_and_output_index(
                transaction_id_invalid_hex_char, output_index)
        output_id_missing_0x_prefix = '52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c6492a000000000000'
        with self.assertRaises(ValueError):
            OutputId(output_id_missing_0x_prefix)
        output_id_invalid_hex_char = '0xz2fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c6492a000000000000'
        with self.assertRaises(ValueError):
            OutputId(output_id_invalid_hex_char)
        output_id_invalid_hex_prefix = '0052fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c6492a000000000000'
        with self.assertRaises(ValueError):
            OutputId(output_id_invalid_hex_prefix)


def test_hex_utf8():
    utf8_data = "Don't panic!"
    hex_data = '0x446f6e27742070616e696321'
    assert utf8_to_hex(utf8_data) == hex_data
    assert hex_to_utf8(hex_data) == utf8_data


def test_irc_27():
    metadata = Irc27Metadata(
        "video/mp4",
        "https://ipfs.io/ipfs/QmPoYcVm9fx47YXNTkhpMEYSxCD3Bqh7PJYr7eo5YjLgiT",
        "Shimmer OG NFT",
        description="The original Shimmer NFT"
    )
    metadata_dict = {
        "standard": "IRC27",
        "version": metadata.version,
        "type": metadata.type,
        "uri": metadata.uri,
        "name": metadata.name,
        "collectionName": metadata.collectionName,
        "royalties": metadata.royalties,
        "issuerName": metadata.issuerName,
        "description": metadata.description,
        "attributes": metadata.attributes
    }
    metadata_deser = Irc27Metadata.from_dict(metadata_dict)
    assert metadata == metadata_deser


def test_irc_30():
    metadata = Irc30Metadata(
        "FooCoin",
        "FOO",
        3,
        description="FooCoin is the utility and governance token of FooLand, \
                a revolutionary protocol in the play-to-earn crypto gaming field.",
        url="https://foocoin.io/",
        logoUrl="https://ipfs.io/ipfs/QmR36VFfo1hH2RAwVs4zVJ5btkopGip5cW7ydY4jUQBrkR"
    )
    metadata_dict = {
        "standard": "IRC30",
        "name": metadata.name,
        "description": metadata.description,
        "decimals": metadata.decimals,
        "symbol": metadata.symbol,
        "url": metadata.url,
        "logoUrl": metadata.logoUrl
    }
    metadata_deser = Irc30Metadata.from_dict(metadata_dict)
    assert metadata == metadata_deser


def test_output_id_hashing():
    output_id = OutputId(
        '0x0000000000000000000000000000000000000000000000000000000000000000000000000000')
    assert Utils.compute_account_id(
        output_id) == '0x0ebc2867a240719a70faacdfc3840e857fa450b37d95297ac4f166c2f70c3345'
    assert Utils.compute_delegation_id(
        output_id) == '0x0ebc2867a240719a70faacdfc3840e857fa450b37d95297ac4f166c2f70c3345'
    assert Utils.compute_nft_id(
        output_id) == '0x0ebc2867a240719a70faacdfc3840e857fa450b37d95297ac4f166c2f70c3345'
