import os

from dotenv import load_dotenv

# pylint: disable=no-name-in-module
from iota_sdk import (ClientOptions, CoinType, StrongholdSecretManager, Wallet, WalletOptions, Bip44,
                      migrate_stronghold_snapshot_v2_to_v3)

load_dotenv()

v2_path = "../../../sdk/tests/wallet/fixtures/v2.stronghold"
v3_path = "./v3.stronghold"
node_url = os.environ.get('NODE_URL', 'https://api.testnet.shimmer.network')
client_options = ClientOptions(nodes=[node_url])
bib_path = Bip44(
    coin_type=CoinType.SHIMMER
)


try:
    secret_manager = StrongholdSecretManager(v2_path, "current_password")
    # This should fail with error, migration required.

    wallet_options = WalletOptions(
        None,
        None,
        bib_path,
        client_options,
        secret_manager,
        os.environ.get('WALLET_DB_PATH'))
    wallet = Wallet(wallet_options)
except ValueError as e:
    print(e)

migrate_stronghold_snapshot_v2_to_v3(
    v2_path,
    "current_password",
    "wallet.rs",
    100,
    v3_path,
    "new_password")

secret_manager = StrongholdSecretManager(v3_path, "new_password")

wallet_options = WalletOptions(None, None, bib_path, client_options, secret_manager, os.environ.get('WALLET_DB_PATH'))

# This shouldn't fail anymore as snapshot has been migrated.
wallet = Wallet(wallet_options)
