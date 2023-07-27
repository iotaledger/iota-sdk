from iota_sdk import Wallet
from dotenv import load_dotenv
import os

# In this example we will list transactions

# This example uses secrets in environment variables for simplicity which
# should not be done in production.
load_dotenv()

wallet = Wallet(os.environ['WALLET_DB_PATH'])

account = wallet.get_account('Alice')
account.sync({'syncIncomingTransactions': True})

# All transactions sent from the the account
transactions = account.transactions()
print('Sent transactions:')
for transaction in transactions:
    print(transaction.transactionId)


# Incoming transactions
incoming_transactions = account.incoming_transactions()
print('Received transactions:')
for transaction in incoming_transactions:
    print(transaction.transactionId)
