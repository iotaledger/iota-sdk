import json
import os

from dotenv import load_dotenv
from iota_sdk import Wallet, WalletOptions

# This example checks the balance of a wallet.

# This example uses secrets in environment variables for simplicity which
# should not be done in production.
load_dotenv()

wallet = Wallet(WalletOptions(storage_path=os.environ.get('WALLET_DB_PATH')))

# Sync wallet with the node
_balance = wallet.sync()

# Just calculate the balance with the known state
balance = wallet.get_balance()
print(f'Balance {json.dumps(balance.to_dict(), indent=4)}')
