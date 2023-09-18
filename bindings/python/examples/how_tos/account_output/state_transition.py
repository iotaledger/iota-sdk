import os

from dotenv import load_dotenv

from iota_sdk import Wallet, FilterOptions, utf8_to_hex

load_dotenv()

# In this example we will update the state metadata of an account output.

NEW_STATE_METADATA = 'updated state metadata 1'

if 'WALLET_DB_PATH' not in os.environ:
    raise Exception(".env WALLET_DB_PATH is undefined, see .env.example")

if 'STRONGHOLD_PASSWORD' not in os.environ:
    raise Exception(".env STRONGHOLD_PASSWORD is undefined, see .env.example")

if 'EXPLORER_URL' not in os.environ:
    raise Exception(".env EXPLORER_URL is undefined, see .env.example")

wallet = Wallet(os.environ['WALLET_DB_PATH'])

account = wallet.get_account('Alice')

# Sync account with the node
balance = account.sync()

if len(balance.accounts) == 0:
    raise Exception("No Account available in account 'Alice'")

account_id = balance.accounts[0]

account_output_data = account.unspent_outputs(
    FilterOptions(accountIds=[account_id]))[0]
print(
    f"Account {account_id} found in unspent output: {account_output_data.outputId}")

account_output = account_output_data.output
updated_account_output = wallet.get_client().build_account_output(
    account_id,
    unlock_conditions=account_output.unlockConditions,
    state_index=int(account_output.stateIndex) + 1,
    state_metadata=utf8_to_hex(NEW_STATE_METADATA),
    foundry_counter=account_output.foundryCounter,
    immutable_features=account_output.immutableFeatures,
    features=account_output.features,
)

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

print('Sending transaction...')
transaction = account.send_outputs([updated_account_output])
print(f'Transaction sent: {transaction.transactionId}')

# Wait for transaction to get included
blockId = account.retry_transaction_until_included(
    transaction.transactionId)
print(f'Block included: {os.environ["EXPLORER_URL"]}/block/{blockId}')
