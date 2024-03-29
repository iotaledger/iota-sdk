import os

from dotenv import load_dotenv

from iota_sdk import CreateNativeTokenParams, Wallet, Irc30Metadata

load_dotenv()

# In this example we will create native tokens

wallet = Wallet(os.environ['WALLET_DB_PATH'])

account = wallet.get_account('Alice')

if 'STRONGHOLD_PASSWORD' not in os.environ:
    raise Exception(".env STRONGHOLD_PASSWORD is undefined, see .env.example")

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

# Sync account with the node
balance = account.sync()

# We can first check if we already have an alias in our account, because
# an alias can have many foundry outputs and therefore we can reuse an
# existing one.
if not balance.aliases:
    # If we don't have an alias, we need to create one
    transaction = account.create_alias_output(None, None)
    print(f'Transaction sent: {transaction.transactionId}')

    # Wait for transaction to get included
    blockId = account.retry_transaction_until_included(
        transaction.transactionId)
    print(f'Block included: {os.environ["EXPLORER_URL"]}/block/{blockId}')

    account.sync()
    print("Account synced")

print('Preparing transaction to create native token...')

metadata = Irc30Metadata(
    "My Native Token", "MNT", 10, description="A native token to test the iota-sdk."
)

params = CreateNativeTokenParams(
    100,
    100,
    metadata.as_hex(),
)

prepared_transaction = account.prepare_create_native_token(params, None)
transaction = prepared_transaction.send()
print(f'Transaction sent: {transaction.transactionId}')

# Wait for transaction to get included
blockId = account.retry_transaction_until_included(transaction.transactionId)
print(f'Block included: {os.environ["EXPLORER_URL"]}/block/{blockId}')

print(f'Created token: {prepared_transaction.token_id()}')

# Ensure the account is synced after creating the native token.
account.sync()
print('Account synced')
