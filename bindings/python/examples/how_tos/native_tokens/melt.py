import os

from dotenv import load_dotenv
from iota_sdk import Wallet, WalletOptions

load_dotenv()

# In this example we will decrease the native token supply

wallet = Wallet(WalletOptions(storage_path=os.environ.get('WALLET_DB_PATH')))

# Sync wallet with the node
balance = wallet.sync()

# Find first foundry and corresponding token id
token_id = balance.foundries[0]

available_balance = balance.native_tokens[token_id].available
print(f'Balance before melting: {available_balance}')

if 'STRONGHOLD_PASSWORD' not in os.environ:
    raise Exception(".env STRONGHOLD_PASSWORD is undefined, see .env.example")

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

melt_amount = 10

# Send transaction.
transaction = wallet.melt_native_token(token_id, melt_amount)
print(f'Transaction sent: {transaction.transaction_id}')

# Wait for transaction to get accepted
block_id = wallet.wait_for_transaction_acceptance(
    transaction.transaction_id)
print(f'Tx accepted in block: {os.environ["EXPLORER_URL"]}/block/{block_id}')

balance = wallet.sync()
available_balance = balance.native_tokens[token_id].available
print(f'Balance after melting: {available_balance}')
