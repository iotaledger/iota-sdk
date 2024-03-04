import json
import os

from dotenv import load_dotenv
from iota_sdk import ClientOptions, CoinType, Wallet, WalletOptions, Bip44

load_dotenv()

# This example restores the wallet from a stronghold.

node_url = os.environ.get('NODE_URL', 'https://api.testnet.shimmer.network')
client_options = ClientOptions(nodes=[node_url])

bib_path = Bip44(
    coin_type=CoinType.SHIMMER
)

wallet_options = WalletOptions(
    None,
    None,
    bib_path,
    client_options,
    'Placeholder',
    './restore-backup-database')
wallet = Wallet(wallet_options)

if 'STRONGHOLD_PASSWORD' not in os.environ:
    raise Exception(".env STRONGHOLD_PASSWORD is undefined, see .env.example")

wallet.restore_from_stronghold_snapshot(
    "backup.stronghold",
    os.environ['STRONGHOLD_PASSWORD'])

print(f'Restored wallet: {json.dumps(wallet, indent=4)}')
