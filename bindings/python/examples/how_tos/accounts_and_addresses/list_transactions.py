from iota_sdk import Wallet

import json

# In this example we will list the sent transactions

wallet = Wallet('./alice-database')

account = wallet.get_account('Alice')
account.sync({ 'syncIncomingTransactions': True })

# All transactions sent from the the account
transactions = account.transactions()
print('Sent transactions:')
for transaction in transactions:
    print(transaction['transactionId'])


# Incoming transactions
incoming_transactions = account.incoming_transactions()
print('Received transactions:')
for transaction in incoming_transactions:
    print(transaction[0])
