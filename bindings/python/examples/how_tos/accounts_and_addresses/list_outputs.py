from iota_sdk import Wallet
from dotenv import load_dotenv
import os

# In this example we will get outputs stored in the account

# This example uses secrets in environment variables for simplicity which
# should not be done in production.
load_dotenv()

wallet = Wallet(os.environ['WALLET_DB_PATH'])

account = wallet.get_account('Alice')
account.sync()

# All outputs stored in the account
outputs = account.outputs()

# Print all output ids
print('Output ids:')
for output in outputs:
    print(output.outputId)

# All unspent outputs stored in the account
outputs = account.unspent_outputs()

# Print all unspent output ids
print('Unspent output ids:')
for output in outputs:
    print(output.outputId)
