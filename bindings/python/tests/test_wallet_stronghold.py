# Copyright 2024 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

import shutil
import unittest
from iota_sdk import Wallet, StrongholdSecretManager, CoinType, ClientOptions


class WalletStronghold(unittest.TestCase):
    def test_wallet_stronghold(self):
        db_path = './test_wallet_stronghold'
        shutil.rmtree(db_path, ignore_errors=True)

        client_options = ClientOptions(nodes=[])

        secret_manager = StrongholdSecretManager('./test_wallet_stronghold/wallet.stronghold',
                                                 'some_hopefully_secure_password')

        wallet = Wallet(db_path,
                        client_options, CoinType.IOTA, secret_manager)
        wallet.store_mnemonic(
            "acoustic trophy damage hint search taste love bicycle foster cradle brown govern endless depend situate athlete pudding blame question genius transfer van random vast")

        account = wallet.create_account('Alice')
        addresses = account.addresses()
        assert 'smr1qpg2xkj66wwgn8p2ggnp7p582gj8g6p79us5hve2tsudzpsr2ap4sp36wye' == addresses[
            0].address

        seed = wallet.get_secret_manager().get_seed()
        assert seed == '0x65d378f26a101366d2b2bc982de128382f260205a8b99266fdcee14cc12f4680eb66171c27be01066c3ea30c9c0b87e27fb90f8cab9ac7b8e205f259d275240f'

        # Destroy the wallet
        wallet.destroy()
        shutil.rmtree(db_path, ignore_errors=True)
