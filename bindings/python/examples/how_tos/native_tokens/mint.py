import os

from dotenv import load_dotenv

from iota_sdk import Wallet, WalletOptions

load_dotenv()

# In this example we will mint native tokens

wallet = Wallet(WalletOptions(storage_path=os.environ.get('WALLET_DB_PATH')))

# Sync account with the node
balance = wallet.sync()

# Find first foundry and corresponding token id
token_id = balance.foundries[0]

available_balance = int(
    [native_balance for native_balance in balance.native_tokens if native_balance.token_id == token_id][0].available, 0)
print(f'Balance before minting: {available_balance}')

if 'STRONGHOLD_PASSWORD' not in os.environ:
    raise Exception(".env STRONGHOLD_PASSWORD is undefined, see .env.example")

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

mint_amount = 10

# Send transaction.
transaction = wallet.mint_native_token(token_id, mint_amount)
print(f'Transaction sent: {transaction.transaction_id}')

# Wait for transaction to get included
block_id = wallet.reissue_transaction_until_included(
    transaction.transaction_id)
print(f'Block included: {os.environ["EXPLORER_URL"]}/block/{block_id}')

balance = wallet.sync()
available_balance = int(
    [native_balance for native_balance in balance.native_tokens if native_balance.token_id == token_id][0].available, 0)
print(f'Balance after minting: {available_balance}')
