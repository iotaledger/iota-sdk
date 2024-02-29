import os

from dotenv import load_dotenv
from iota_sdk import Wallet, WalletOptions

# This example uses secrets in environment variables for simplicity which
# should not be done in production.
load_dotenv()

# This example prints the wallet address.

if 'WALLET_DB_PATH' not in os.environ:
    raise Exception(".env WALLET_DB_PATH is undefined, see .env.example")

wallet = Wallet(WalletOptions(storage_path=os.environ.get('WALLET_DB_PATH')))

address = wallet.address()

print(address)
