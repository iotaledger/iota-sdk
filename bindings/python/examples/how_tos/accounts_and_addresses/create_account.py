from iota_sdk import Wallet, StrongholdSecretManager, CoinType
from dotenv import load_dotenv
import json
import os
import sys

load_dotenv()

# This example creates a new database and account

node_url = os.environ.get('NODE_URL', 'https://api.testnet.shimmer.network')
client_options = {
    'nodes': [node_url],
}

# Shimmer coin type
coin_type = CoinType.SHIMMER

if 'STRONGHOLD_PASSWORD' not in os.environ:
    raise Exception(".env STRONGHOLD_PASSWORD is undefined, see .env.example")

secret_manager = StrongholdSecretManager(
    os.environ['STRONGHOLD_SNAPSHOT_PATH'], os.environ['STRONGHOLD_PASSWORD'])

wallet = Wallet(os.environ['WALLET_DB_PATH'], client_options, coin_type, secret_manager)

if 'MNEMONIC' not in os.environ:
    raise Exception(".env MNEMONIC is undefined, see .env.example")

# Store the mnemonic in the Stronghold snapshot, this only needs to be done once
wallet.store_mnemonic(os.environ['MNEMONIC'])

account = wallet.create_account('Alice')
print("Account created:", account.get_metadata())
