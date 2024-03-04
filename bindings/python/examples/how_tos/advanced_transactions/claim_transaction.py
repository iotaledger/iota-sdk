import os

from dotenv import load_dotenv
from iota_sdk import Wallet, WalletOptions

# This example uses secrets in environment variables for simplicity which
# should not be done in production.
load_dotenv()

# In this example we will claim outputs that have additional unlock
# conditions as expiration or storage deposit return.

for env_var in ['WALLET_DB_PATH', 'STRONGHOLD_PASSWORD']:
    if env_var not in os.environ:
        raise Exception(f'.env {env_var} is undefined, see .env.example')

wallet = Wallet(WalletOptions(storage_path=os.environ.get('WALLET_DB_PATH')))

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

# Sync wallet with the node
wallet.sync()

# Only the unspent outputs in the wallet
output_ids = wallet.claimable_outputs('All')

print('Available outputs to claim:')
for output_id in output_ids:
    print(f'{output_id}')

transaction = wallet.claim_outputs(output_ids)
print(f'Transaction sent: {transaction.transaction_id}')

wallet.wait_for_transaction_acceptance(
    transaction.transaction_id)
print(
    f'Tx accepted: {os.environ["EXPLORER_URL"]}/transactions/{transaction.transaction_id}')
