from iota_sdk import Wallet

# This example checks the balance of an account.

wallet = Wallet('./alice-database')

account = wallet.get_account('Alice')

# Sync account with the node
response = account.sync()
print(f'Synced: {response}')
print()

# Just calculate the balance with the known state
balance = account.get_balance()
print(f'Balance: {balance}')
print()

print('Addresses:')
for address in account.addresses():
    print(' - {}'.format(address['address']))
