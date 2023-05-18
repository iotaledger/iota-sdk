from iota_sdk import Wallet

# In this example we will get outputs stored in the account

wallet = Wallet('./alice-database')

account = wallet.get_account('Alice')
account.sync()

# All outputs stored in the account
outputs = account.outputs()

# Print all output ids
print('Output ids:')
for output in outputs:
    print(output['outputId'])

# All unspent outputs stored in the account
outputs = account.outputs()

# Print all unspent output ids
print('Unspent output ids:')
for output in outputs:
    print(output['outputId'])
