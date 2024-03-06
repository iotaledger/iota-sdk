import os
from dotenv import load_dotenv
from iota_sdk import Wallet, WalletOptions

load_dotenv()

# In this example we will burn native tokens

wallet = Wallet(WalletOptions(storage_path=os.environ.get('WALLET_DB_PATH')))

# Sync wallet with the node
balance = wallet.sync()

if 'STRONGHOLD_PASSWORD' not in os.environ:
    raise Exception(".env STRONGHOLD_PASSWORD is undefined, see .env.example")

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

# Find native token with enough balance
token_id, token = next(
    (k, v) for k, v in balance.native_tokens.items() if v.available >= 10)
print(f'Balance before burning: {token.available}')

burn_amount = 1

# Send transaction.
transaction = wallet.prepare_burn_native_token(
    token_id, burn_amount).send()
print(f'Transaction sent: {transaction.transaction_id}')

wallet.wait_for_transaction_acceptance(
    transaction.transaction_id)
print(
    f'Tx accepted: {os.environ["EXPLORER_URL"]}/transactions/{transaction.transaction_id}')

balance = wallet.sync()
available_balance = balance.native_tokens[token_id].available
print(f'Balance after burning: {available_balance}')
