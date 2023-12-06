import os

from dotenv import load_dotenv

from iota_sdk import Wallet

load_dotenv()

# In this example we will destroy an account output

wallet = Wallet(os.environ['WALLET_DB_PATH'])

# Sync account with the node
balance = wallet.sync()

# We try to destroy the first account in the account
account_id = balance.accounts[0]

if 'STRONGHOLD_PASSWORD' not in os.environ:
    raise Exception(".env STRONGHOLD_PASSWORD is undefined, see .env.example")

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

# Send transaction.
transaction = account.prepare_destroy_account(account_id).send()
print(f'Block sent: {os.environ["EXPLORER_URL"]}/block/{transaction.block_id}')
