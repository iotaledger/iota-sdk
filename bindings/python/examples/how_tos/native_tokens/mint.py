from iota_sdk import Wallet, utf8_to_hex
from dotenv import load_dotenv
import time
import os

load_dotenv()

# In this example we will mint native tokens

wallet = Wallet('./alice-database')

account = wallet.get_account('Alice')

if 'STRONGHOLD_PASSWORD' not in os.environ:
    raise Exception(".env STRONGHOLD_PASSWORD is undefined, see .env.example")

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

# Sync account with the node
response = account.sync()

print('Preparing alias output transaction...')

transaction = account.prepare_create_alias_output(None, None).send()
print(f'Transaction sent: {transaction["transactionId"]}')

# Wait for transaction to get included
blockId = account.retry_transaction_until_included(transaction['transactionId'])
print(f'Block included: {os.environ["EXPLORER_URL"]}/block/{blockId}')

account.sync()
print("Account synced")

print('Preparing minting transaction...')

params = {
    "circulatingSupply": hex(100),
    "maximumSupply": hex(100),
    "foundryMetadata": utf8_to_hex('Hello, World!'),
}

prepared_transaction = account.prepare_mint_native_token(params, None)
transaction = prepared_transaction.send()
print(f'Transaction sent: {transaction["transactionId"]}')

# Wait for transaction to get included
blockId = account.retry_transaction_until_included(transaction['transactionId'])
print(f'Block included: {os.environ["EXPLORER_URL"]}/block/{blockId}')

print(f'Minted token: {prepared_transaction.token_id()}')

# Ensure the account is synced after minting.
account.sync()
print('Account synced')
