import json
import os

from dotenv import load_dotenv

from iota_sdk import Wallet

# This example checks the balance of an account.

# This example uses secrets in environment variables for simplicity which
# should not be done in production.
load_dotenv()

wallet = Wallet(os.environ['WALLET_DB_PATH'])

# Sync account with the node
_balance = wallet.sync()

# Just calculate the balance with the known state
balance = account.get_balance()
print(f'Balance {json.dumps(balance.to_dict(), indent=4)}')
