from iota_sdk import Wallet, utf8_to_hex
from dotenv import load_dotenv
import time
import os

load_dotenv()

# In this example we will create native tokens

wallet = Wallet('./alice-database')

account = wallet.get_account('Alice')

if 'STRONGHOLD_PASSWORD' not in os.environ:
    raise Exception(".env STRONGHOLD_PASSWORD is undefined, see .env.example")

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

# Sync account with the node
response = account.sync()

# We can first check if we already have an alias in our account, because an alias can have many foundry outputs and therefore we can reuse an existing one
if len(account.aliases) == 0:
    # If we don't have an alias, we need to create one
    transaction = account.prepare_create_alias_output(None, None).send()
    print(f'Transaction sent: {transaction["transactionId"]}')

    # Wait for transaction to get included
    blockId = account.retry_transaction_until_included(transaction['transactionId'])
    print(f'Block included: {os.environ["EXPLORER_URL"]}/block/{blockId}')

    account.sync()
    print("Account synced")

print('Preparing transaction to create native token...')

params = {
    "circulatingSupply": hex(100),
    "maximumSupply": hex(100),
    "foundryMetadata": utf8_to_hex('Hello, World!'),
}

prepared_transaction = account.prepare_create_native_token(params, None)
transaction = prepared_transaction.send()
print(f'Transaction sent: {transaction["transactionId"]}')

# Wait for transaction to get included
blockId = account.retry_transaction_until_included(transaction['transactionId'])
print(f'Block included: {os.environ["EXPLORER_URL"]}/block/{blockId}')

print(f'Created token: {prepared_transaction.token_id()}')

# Ensure the account is synced after creating the native token.
account.sync()
print('Account synced')
