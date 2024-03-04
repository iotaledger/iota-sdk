import os

from dotenv import load_dotenv
from iota_sdk import Wallet, WalletOptions

load_dotenv()

# In this example we will mint native tokens

wallet = Wallet(WalletOptions(storage_path=os.environ.get('WALLET_DB_PATH')))

# Sync wallet with the node
balance = wallet.sync()

# Find first foundry and corresponding token id
token_id = balance.foundries[0]

available_balance = balance.native_tokens[token_id].available
print(f'Balance before minting: {available_balance}')

if 'STRONGHOLD_PASSWORD' not in os.environ:
    raise Exception(".env STRONGHOLD_PASSWORD is undefined, see .env.example")

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

mint_amount = 10

# Send transaction.
transaction = wallet.mint_native_token(token_id, mint_amount)
print(f'Transaction sent: {transaction.transaction_id}')

wallet.wait_for_transaction_acceptance(
    transaction.transaction_id)
print(
    f'Tx accepted: {os.environ["EXPLORER_URL"]}/transactions/{transaction.transaction_id}')

balance = wallet.sync()
available_balance = balance.native_tokens[token_id].available
print(f'Balance after minting: {available_balance}')
