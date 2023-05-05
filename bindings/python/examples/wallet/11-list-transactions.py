from iota_sdk import Wallet

# In this example we will list the sent transactions

wallet = Wallet('./alice-database')

account = wallet.get_account('Alice')

# All transactions sent from the the account
transactions = account.transactions()
print(f'Transactions: {transactions}')

# Pending transactions
pending_transactions = account.pending_transactions()
print(f'Pending transactions: {pending_transactions}')
