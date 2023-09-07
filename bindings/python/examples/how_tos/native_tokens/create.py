from iota_sdk import Wallet, utf8_to_hex, CreateNativeTokenParams
from dotenv import load_dotenv
import os

load_dotenv()

# In this example we will create native tokens

wallet = Wallet(os.environ['WALLET_DB_PATH'])

account = wallet.get_account('Alice')

if 'STRONGHOLD_PASSWORD' not in os.environ:
    raise Exception(".env STRONGHOLD_PASSWORD is undefined, see .env.example")

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

# Sync account with the node
balance = account.sync()

# We can first check if we already have an account in our account, because
# an account can have many foundry outputs and therefore we can reuse an
# existing one.
if not balance.accounts:
    # If we don't have an account, we need to create one
    transaction = account.prepare_create_account_output(None, None).send()
    print(f'Transaction sent: {transaction.transaction_id}')

    # Wait for transaction to get included
    block_id = account.reissue_transaction_until_included(
        transaction.transaction_id)
    print(f'Block included: {os.environ["EXPLORER_URL"]}/block/{block_id}')

    account.sync()
    print("Account synced")

print('Preparing transaction to create native token...')

params = CreateNativeTokenParams(
    100,
    100,
    utf8_to_hex('Hello, World!'),
)

prepared_transaction = account.prepare_create_native_token(params, None)
transaction = prepared_transaction.send()
print(f'Transaction sent: {transaction.transaction_id}')

# Wait for transaction to get included
block_id = account.reissue_transaction_until_included(
    transaction.transaction_id)
print(f'Block included: {os.environ["EXPLORER_URL"]}/block/{block_id}')

print(f'Created token: {prepared_transaction.token_id()}')

# Ensure the account is synced after creating the native token.
account.sync()
print('Account synced')
