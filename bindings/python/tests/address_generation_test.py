# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

import shutil
from iota_sdk import Wallet, MnemonicSecretManager, CoinType, ClientOptions, WalletOptions, Bip44


def test_address_generation_iota():
    db_path = './test_address_generation_iota'
    shutil.rmtree(db_path, ignore_errors=True)

    client_options = ClientOptions(nodes=[])

    secret_manager = MnemonicSecretManager(
        "acoustic trophy damage hint search taste love bicycle foster cradle brown govern endless depend situate athlete pudding blame question genius transfer van random vast")

    bip_path = Bip44(
        coin_type=CoinType.IOTA
    )
    wallet_options = WalletOptions(
        None,
        None,
        bip_path,
        client_options,
        secret_manager,
        db_path)
    wallet = Wallet(wallet_options)

    address = wallet.address()

    assert 'smr1qpg2xkj66wwgn8p2ggnp7p582gj8g6p79us5hve2tsudzpsr2ap4sp36wye' == address
    shutil.rmtree(db_path, ignore_errors=True)


def test_address_generation_shimmer():
    db_path = './test_address_generation_shimmer'
    shutil.rmtree(db_path, ignore_errors=True)

    client_options = ClientOptions(nodes=[])

    secret_manager = MnemonicSecretManager(
        "acoustic trophy damage hint search taste love bicycle foster cradle brown govern endless depend situate athlete pudding blame question genius transfer van random vast")

    bip_path = Bip44(
        coin_type=CoinType.SHIMMER
    )
    wallet_options = WalletOptions(
        None,
        None,
        bip_path,
        client_options,
        secret_manager,
        db_path)
    wallet = Wallet(wallet_options)

    address = wallet.address()

    assert 'smr1qzev36lk0gzld0k28fd2fauz26qqzh4hd4cwymlqlv96x7phjxcw6ckj80y' == address
    shutil.rmtree(db_path, ignore_errors=True)
