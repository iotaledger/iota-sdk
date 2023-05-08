from iota_sdk import Wallet, StrongholdSecretManager, CoinType
from dotenv import load_dotenv
import os

load_dotenv()

# This example creates a new database and account

node_url = os.environ.get('NODE_URL', 'https://api.testnet.shimmer.network')
client_options = {
    'nodes': [node_url],
}

# Shimmer coin type
coin_type = CoinType.SHIMMER

if 'STRONGHOLD_PASSWORD' not in os.environ:
    print(".env STRONGHOLD_PASSWORD is undefined, see .env.example")
    sys.exit(1)

secret_manager = StrongholdSecretManager(
    "wallet.stronghold", os.environ['STRONGHOLD_PASSWORD'])

wallet = Wallet('./backup-database', client_options,
                    coin_type, secret_manager)

if 'NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1' not in os.environ:
    print(".env mnemonic is undefined, see .env.example")
    sys.exit(1)

# Store the mnemonic in the Stronghold snapshot, this only needs to be done once
account = wallet.store_mnemonic(os.environ['NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1'])

accounts = wallet.create_account('Alice')

wallet.backup("backup.stronghold", os.environ['STRONGHOLD_PASSWORD'])
print(f'Created backup')
