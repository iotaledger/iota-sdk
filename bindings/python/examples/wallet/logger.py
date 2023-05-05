from iota_sdk import Wallet, StrongholdSecretManager, init_logger, CoinType
from dotenv import load_dotenv
import json
import os

load_dotenv()

# This example creates a new database and account and write debug logs in `wallet.log`.

log_config = {
    "name": './wallet.log',
    "levelFilter": 'debug',
    "targetExclusions": ["h2", "hyper", "rustls"]
}

# Init the logger
init_logger(json.dumps(log_config))

node_url = os.environ.get('NODE_URL', 'https://api.testnet.shimmer.network')
client_options = {
    'nodes': [node_url],
}

# Shimmer coin type
coin_type = CoinType.SHIMMER

if 'STRONGHOLD_PASSWORD' not in os.environ:
    print(".env STRONGHOLD_PASSWORD is undefined, see .env.example")
    sys.exit(1)

secret_manager = StrongholdSecretManager("wallet.stronghold", os.environ["STRONGHOLD_PASSWORD"])

wallet = Wallet('./alice-database', client_options,
                    coin_type, secret_manager)

if 'NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1' not in os.environ:
    print(".env mnemonic is undefined, see .env.example")
    sys.exit(1)

# Store the mnemonic in the Stronghold snapshot, this only needs to be done once
account = wallet.store_mnemonic(os.environ["NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1"])

account = wallet.create_account('Alice')
print(json.dumps(account, indent=4))
