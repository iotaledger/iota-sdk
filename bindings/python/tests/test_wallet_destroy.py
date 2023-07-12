# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from iota_sdk import Wallet, MnemonicSecretManager, CoinType, WalletError
import shutil
import unittest


class WalletDestroy(unittest.TestCase):
    def test_wallet_destroy(self):
        db_path = './test_wallet_destroy'
        shutil.rmtree(db_path, ignore_errors=True)

        client_options = {
            'nodes': [],
        }

        secret_manager = MnemonicSecretManager(
            "acoustic trophy damage hint search taste love bicycle foster cradle brown govern endless depend situate athlete pudding blame question genius transfer van random vast")

        wallet = Wallet(db_path,
                        client_options, CoinType.IOTA, secret_manager)

        account = wallet.create_account('Alice')

        addresses = account.addresses()
        assert 'smr1qpg2xkj66wwgn8p2ggnp7p582gj8g6p79us5hve2tsudzpsr2ap4sp36wye' == addresses[
            0]['address']

        # Destroy the wallet
        wallet.destroy()

        # Afterwards destroying we can recreate the wallet again
        wallet = Wallet(db_path,
                        client_options, CoinType.IOTA, secret_manager)

        account = wallet.get_account('Alice')

        addresses = account.addresses()
        assert 'smr1qpg2xkj66wwgn8p2ggnp7p582gj8g6p79us5hve2tsudzpsr2ap4sp36wye' == addresses[
            0]['address']
        shutil.rmtree(db_path, ignore_errors=True)

    def test_wallet_destroy_error(self):
        db_path = './test_wallet_destroy_error'
        shutil.rmtree(db_path, ignore_errors=True)

        client_options = {
            'nodes': [],
        }

        secret_manager = MnemonicSecretManager(
            "acoustic trophy damage hint search taste love bicycle foster cradle brown govern endless depend situate athlete pudding blame question genius transfer van random vast")

        wallet = Wallet(db_path,
                        client_options, CoinType.IOTA, secret_manager)

        # Destroy the wallet
        wallet.destroy()

        with self.assertRaises(WalletError):
            wallet.create_account('Alice')

        shutil.rmtree(db_path, ignore_errors=True)
