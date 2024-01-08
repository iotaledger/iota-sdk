import os

from dotenv import load_dotenv

from iota_sdk import Wallet, WalletOptions

# In this example we will list transactions

# This example uses secrets in environment variables for simplicity which
# should not be done in production.
load_dotenv()

wallet = Wallet(WalletOptions(storage_path=os.environ.get('WALLET_DB_PATH')))
wallet.sync({'syncIncomingTransactions': True})

# All transactions sent from the wallet
transactions = wallet.transactions()
print('Sent transactions:')
for transaction in transactions:
    print(transaction.transaction_id)


# Incoming transactions
incoming_transactions = wallet.incoming_transactions()
print('Received transactions:')
for transaction in incoming_transactions:
    print(transaction.transaction_id)
