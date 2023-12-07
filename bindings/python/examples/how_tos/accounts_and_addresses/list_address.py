import os

from dotenv import load_dotenv

from iota_sdk import Wallet, WalletOptions

# This example lists the wallet address.

# This example uses secrets in environment variables for simplicity which
# should not be done in production.
load_dotenv()

wallet = Wallet(WalletOptions(storage_path=os.environ.get('WALLET_DB_PATH')))

address = wallet.address()

print(address)
