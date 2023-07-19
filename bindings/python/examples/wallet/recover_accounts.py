from iota_sdk import Wallet, StrongholdSecretManager, CoinType, ClientOptions
from dotenv import load_dotenv
import json
import os

load_dotenv()

# This example searches for accounts with unspent outputs.

node_url = os.environ.get('NODE_URL', 'https://api.testnet.shimmer.network')
client_options = ClientOptions(nodes=[node_url])

# Shimmer coin type
coin_type = CoinType.SHIMMER

if 'STRONGHOLD_PASSWORD' not in os.environ:
    raise Exception(".env STRONGHOLD_PASSWORD is undefined, see .env.example")

secret_manager = StrongholdSecretManager(
    os.environ['STRONGHOLD_SNAPSHOT_PATH'], os.environ['STRONGHOLD_PASSWORD'])

wallet = Wallet(
    os.environ['WALLET_DB_PATH'],
    client_options,
    coin_type,
    secret_manager)

if 'MNEMONIC' not in os.environ:
    raise Exception(".env MNEMONIC is undefined, see .env.example")

# Store the mnemonic in the Stronghold snapshot, this only needs to be
# done once.
account = wallet.store_mnemonic(os.environ['MNEMONIC'])

# Searches for unspent outputs until no ones are found for 3 accounts in a row
# and checks the addresses for each account until 10 addresses in a row
# have nothing.
accounts = wallet.recover_accounts(0, 3, 10, None)
print(json.dumps(accounts, indent=4))
