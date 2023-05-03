from iota_sdk import Wallet
from dotenv import load_dotenv
import os

load_dotenv()

# In this example we will list the sent transactions

wallet = Wallet('./alice-database')

account = wallet.get_account('Alice')

if 'STRONGHOLD_PASSWORD' not in os.environ:
    print(".env STRONGHOLD_PASSWORD is undefined, see .env.example")
    sys.exit(1)

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

# All transactions sent from the the account
transactions = account.transactions()
print(f'Transactions: {transactions}')

# Pending transactions
pending_transactions = account.pending_transactions()
print(f'Pending transactions: {pending_transactions}')
