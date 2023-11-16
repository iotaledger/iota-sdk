import os

from dotenv import load_dotenv

from iota_sdk import ConsolidationParams, Utils, Wallet

# In this example we will claim all available outputs from an account.

# This example uses secrets in environment variables for simplicity which
# should not be done in production.
load_dotenv()

if 'STRONGHOLD_PASSWORD' not in os.environ:
    raise Exception('.env STRONGHOLD_PASSWORD is undefined, see .env.example')

wallet = Wallet(os.environ['WALLET_DB_PATH'])
wallet.set_stronghold_password(os.environ['STRONGHOLD_PASSWORD'])

account = wallet.get_account('Alice')

# Sync account to make sure account is updated with outputs from previous
# examples.
account.sync()
print('Account synced')

print('Preparing claim transaction...')
claimable = account.claimable_outputs('All')
prepared_transaction = account.prepare_claim_outputs(claimable)
print('Claim transaction:', prepared_transaction.prepared_transaction_data())
transaction = prepared_transaction.send()

# Wait for the consolidation transaction to get confirmed
block_id = account.retry_transaction_until_included(transaction.transactionId)

print(
    f'Transaction included: {os.environ["EXPLORER_URL"]}/block/{block_id}'
)

# Sync account
account.sync()
print('Account synced')
