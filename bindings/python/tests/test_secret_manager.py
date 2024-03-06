# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from iota_sdk import MnemonicSecretManager, SecretManager, CoinType, Utils


def test_secret_manager_address_generation_iota():
    secret_manager = SecretManager(MnemonicSecretManager(
        "acoustic trophy damage hint search taste love bicycle foster cradle brown govern endless depend situate athlete pudding blame question genius transfer van random vast"))

    bech32_hrp = Utils.iota_mainnet_protocol_parameters().bech32_hrp
    address = secret_manager.generate_ed25519_address_as_bech32(CoinType.IOTA, bech32_hrp)

    assert 'iota1qpg2xkj66wwgn8p2ggnp7p582gj8g6p79us5hve2tsudzpsr2ap4skprwjg' == address


def test_secret_manager_address_generation_shimmer():
    secret_manager = SecretManager(MnemonicSecretManager(
        "acoustic trophy damage hint search taste love bicycle foster cradle brown govern endless depend situate athlete pudding blame question genius transfer van random vast"))

    bech32_hrp = Utils.shimmer_mainnet_protocol_parameters().bech32_hrp
    address = secret_manager.generate_ed25519_address_as_bech32(CoinType.SHIMMER, bech32_hrp)

    assert 'smr1qzev36lk0gzld0k28fd2fauz26qqzh4hd4cwymlqlv96x7phjxcw6ckj80y' == address
