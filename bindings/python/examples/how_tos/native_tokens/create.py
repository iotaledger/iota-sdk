import os

from dotenv import load_dotenv
from iota_sdk import CreateNativeTokenParams, Wallet, WalletOptions, Irc30Metadata

load_dotenv()

# In this example we will create native tokens

wallet = Wallet(WalletOptions(storage_path=os.environ.get('WALLET_DB_PATH')))

if 'STRONGHOLD_PASSWORD' not in os.environ:
    raise Exception(".env STRONGHOLD_PASSWORD is undefined, see .env.example")

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

# Sync wallet with the node
balance = wallet.sync()

# We can first check if we already have an account output in our account, because
# an account can have many foundry outputs and therefore we can reuse an
# existing one.
if not balance.accounts:
    # If we don't have an account, we need to create one
    transaction = wallet.create_account_output(None, None)
    print(f'Transaction sent: {transaction.transaction_id}')

    wallet.wait_for_transaction_acceptance(
        transaction.transaction_id)
    print(
        f'Tx accepted: {os.environ["EXPLORER_URL"]}/transactions/{transaction.transaction_id}')

    wallet.sync()
    print("Wallet synced")

print('Preparing transaction to create native token...')

metadata = Irc30Metadata(
    "My Native Token", "MNT", 10, description="A native token to test the iota-sdk."
)

params = CreateNativeTokenParams(
    100,
    100,
    metadata.as_feature(),
)

prepared_transaction = wallet.prepare_create_native_token(params, None)
transaction = prepared_transaction.send()
print(f'Transaction sent: {transaction.transaction_id}')

wallet.wait_for_transaction_acceptance(
    transaction.transaction_id)
print(
    f'Tx accepted: {os.environ["EXPLORER_URL"]}/transactions/{transaction.transaction_id}')

print(f'Created token: {transaction.token_id}')

# Ensure the wallet is synced after creating the native token.
wallet.sync()
print('Wallet synced')
