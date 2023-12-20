import json
import os

from dotenv import load_dotenv

# pylint: disable=no-name-in-module
from iota_sdk import (ClientOptions, CoinType, StrongholdSecretManager, Wallet, WalletOptions, Bip44,
                      init_logger)

load_dotenv()

# This example creates a new database and wallet and write debug logs in
# `wallet.log`.

log_config = {
    "name": './wallet.log',
    "levelFilter": 'debug',
    "targetExclusions": ["h2", "hyper", "rustls"]
}

# Init the logger
init_logger(json.dumps(log_config))

node_url = os.environ.get('NODE_URL', 'https://api.testnet.shimmer.network')
client_options = ClientOptions(nodes=[node_url])

for env_var in ['STRONGHOLD_PASSWORD', 'MNEMONIC']:
    if env_var not in os.environ:
        raise Exception(f'.env {env_var} is undefined, see .env.example')

secret_manager = StrongholdSecretManager(
    "wallet.stronghold",
    os.environ["STRONGHOLD_PASSWORD"])

bib_path = Bip44(
    coin_type=CoinType.SHIMMER
)

wallet_options = WalletOptions(
    None,
    None,
    bib_path,
    client_options,
    secret_manager,
    os.environ.get('WALLET_DB_PATH'))
wallet = Wallet(wallet_options)

# Store the mnemonic in the Stronghold snapshot, this only needs to be
# done once.
wallet.store_mnemonic(os.environ["MNEMONIC"])
