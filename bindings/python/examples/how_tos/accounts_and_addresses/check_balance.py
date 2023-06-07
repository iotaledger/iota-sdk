from iota_sdk import Wallet
import json

# This example checks the balance of an account.

wallet = Wallet('./alice-database')

account = wallet.get_account('Alice')

# Sync account with the node
_balance = account.sync()

# Just calculate the balance with the known state
balance = account.get_balance()
print(f'Balance {json.dumps(balance, indent=4)}')
