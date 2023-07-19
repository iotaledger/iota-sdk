from iota_sdk import Wallet
from dotenv import load_dotenv
import json
import os

# This example checks the balance of an account.

# This example uses secrets in environment variables for simplicity which
# should not be done in production.
load_dotenv()

wallet = Wallet(os.environ['WALLET_DB_PATH'])

account = wallet.get_account('Alice')

# Sync account with the node
_balance = account.sync()

# Just calculate the balance with the known state
balance = account.get_balance()
print(f'Balance {json.dumps(balance.as_dict(), indent=4)}')
