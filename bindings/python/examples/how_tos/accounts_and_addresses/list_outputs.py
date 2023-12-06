import os

from dotenv import load_dotenv

from iota_sdk import Wallet

# In this example we will get outputs stored in the account

# This example uses secrets in environment variables for simplicity which
# should not be done in production.
load_dotenv()

wallet = Wallet(os.environ['WALLET_DB_PATH'])
wallet.sync()

# All outputs stored in the account
outputs = account.outputs()

# Print all output ids
print('Output ids:')
for output in outputs:
    print(output.output_id)

# All unspent outputs stored in the account
outputs = account.unspent_outputs()

# Print all unspent output ids
print('Unspent output ids:')
for output in outputs:
    print(output.output_id)
