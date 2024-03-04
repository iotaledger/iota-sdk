import os

from dotenv import load_dotenv
from iota_sdk import ClientOptions, CoinType, StrongholdSecretManager, SecretManager, Wallet, WalletOptions, Bip44

load_dotenv()

# This example creates a new database and wallet

node_url = os.environ.get('NODE_URL', 'https://api.testnet.shimmer.network')
client_options = ClientOptions(nodes=[node_url])

for env_var in ['STRONGHOLD_PASSWORD', 'MNEMONIC']:
    if env_var not in os.environ:
        raise Exception(f".env {env_var} is undefined, see .env.example")

secret_manager = StrongholdSecretManager(
    os.environ['STRONGHOLD_SNAPSHOT_PATH'], os.environ['STRONGHOLD_PASSWORD'])

# Store the mnemonic in the Stronghold snapshot, this only needs to be
# done once.
SecretManager(secret_manager).store_mnemonic(os.environ['MNEMONIC'])

bip_path = Bip44(
    coin_type=CoinType.SHIMMER
)

wallet_options = WalletOptions(
    None,
    None,
    bip_path,
    client_options,
    secret_manager,
    os.environ.get('WALLET_DB_PATH'))
wallet = Wallet(wallet_options)

# Update the wallet to the latest state
balance = wallet.sync()
print('Generated new wallet')
