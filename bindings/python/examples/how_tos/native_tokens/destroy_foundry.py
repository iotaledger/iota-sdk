import os
from dotenv import load_dotenv
from iota_sdk import Wallet, WalletOptions

load_dotenv()

# In this example we will destroy a foundry

wallet = Wallet(WalletOptions(storage_path=os.environ.get('WALLET_DB_PATH')))

# Sync account with the node
balance = wallet.sync()
print(f'Foundries before destroying: {len(balance.foundries)}')

if 'STRONGHOLD_PASSWORD' not in os.environ:
    raise Exception(".env STRONGHOLD_PASSWORD is undefined, see .env.example")

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

# We try to destroy the first foundry in the account
foundry_id = balance.foundries[0]

# Send transaction.
transaction = wallet.prepare_destroy_foundry(foundry_id).send()
print(f'Transaction sent: {transaction.transaction_id}')

# Wait for transaction to get included
block_id = wallet.reissue_transaction_until_included(
    transaction.transaction_id)
print(f'Block included: {os.environ["EXPLORER_URL"]}/block/{block_id}')

balance = wallet.sync()
print(f'Foundries after destroying: {len(balance.foundries)}')
