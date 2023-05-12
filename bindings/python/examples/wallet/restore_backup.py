from iota_sdk import Wallet, CoinType
from dotenv import load_dotenv
import json
import os

load_dotenv()

# This example restores the wallet from a stronghold.

node_url = os.environ.get('NODE_URL', 'https://api.testnet.shimmer.network')
client_options = {
    'nodes': [node_url],
}

# Shimmer coin type
coin_type = CoinType.SHIMMER

wallet = Wallet('./restore-backup-database', client_options,
                    coin_type, 'Placeholder')

if 'STRONGHOLD_PASSWORD' not in os.environ:
    print(".env STRONGHOLD_PASSWORD is undefined, see .env.example")
    sys.exit(1)

wallet.restore_backup("backup.stronghold", os.environ['STRONGHOLD_PASSWORD'])

accounts = wallet.get_accounts()
print(f'Restored accounts: {json.dumps(accounts, indent=4)}')