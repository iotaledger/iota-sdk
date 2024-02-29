import os

from dotenv import load_dotenv
from iota_sdk import ClientOptions, CoinType, StrongholdSecretManager, Wallet, WalletOptions, Bip44

load_dotenv()

# This example creates a new database and wallet

node_url = os.environ.get('NODE_URL', 'https://api.testnet.shimmer.network')
client_options = ClientOptions(nodes=[node_url])

for env_var in ['STRONGHOLD_PASSWORD', 'MNEMONIC']:
    if env_var not in os.environ:
        raise Exception(f'.env {env_var} is undefined, see .env.example')

secret_manager = StrongholdSecretManager(
    os.environ['STRONGHOLD_SNAPSHOT_PATH'], os.environ['STRONGHOLD_PASSWORD'])

bib_path = Bip44(
    coin_type=CoinType.SHIMMER
)

wallet_options = WalletOptions(
    None,
    None,
    bib_path,
    client_options,
    secret_manager,
    './backup-database')
wallet = Wallet(wallet_options)

# Store the mnemonic in the Stronghold snapshot, this only needs to be
# done once.
wallet.store_mnemonic(os.environ['MNEMONIC'])

wallet.backup_to_stronghold_snapshot(
    "backup.stronghold",
    os.environ['STRONGHOLD_PASSWORD'])
print('Created backup')
