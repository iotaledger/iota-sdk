import os

from dotenv import load_dotenv

from iota_sdk import Wallet, WalletOptions

# In this example we will get outputs stored in the wallet

# This example uses secrets in environment variables for simplicity which
# should not be done in production.
load_dotenv()

wallet = Wallet(WalletOptions(storage_path=os.environ.get('WALLET_DB_PATH')))
wallet.sync()

# All outputs stored in the wallet
outputs = wallet.outputs()

# Print all output ids
print('Output ids:')
for output in outputs:
    print(output.output_id)

# All unspent outputs stored in the wallet
outputs = wallet.unspent_outputs()

# Print all unspent output ids
print('Unspent output ids:')
for output in outputs:
    print(output.output_id)
