# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

import shutil
import unittest
from iota_sdk import Wallet, MnemonicSecretManager, CoinType, ClientOptions, WalletOptions, WalletError, Bip44, Utils


class WalletDestroy(unittest.TestCase):
    def test_wallet_destroy(self):
        db_path = './test_wallet_destroy'
        shutil.rmtree(db_path, ignore_errors=True)

        client_options = ClientOptions(
            nodes=[], protocol_parameters=Utils.iota_mainnet_protocol_parameters())

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
        assert 'iota1qpg2xkj66wwgn8p2ggnp7p582gj8g6p79us5hve2tsudzpsr2ap4skprwjg' == address

        # Destroy the wallet
        wallet.destroy()

        # Afterwards destroying we can recreate the wallet again
        wallet = Wallet(wallet_options)

        address = wallet.address()
        assert 'iota1qpg2xkj66wwgn8p2ggnp7p582gj8g6p79us5hve2tsudzpsr2ap4skprwjg' == address
        shutil.rmtree(db_path, ignore_errors=True)

    def test_wallet_destroy_error(self):
        db_path = './test_wallet_destroy_error'
        shutil.rmtree(db_path, ignore_errors=True)

        client_options = ClientOptions(
            nodes=[], protocol_parameters=Utils.iota_mainnet_protocol_parameters())
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

        # Destroy the wallet
        wallet.destroy()

        with self.assertRaises(WalletError):
            wallet.address()

        shutil.rmtree(db_path, ignore_errors=True)
