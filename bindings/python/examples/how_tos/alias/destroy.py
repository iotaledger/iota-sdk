import os

from dotenv import load_dotenv

from iota_sdk import Wallet

load_dotenv()

# In this example we will destroy an alias output

wallet = Wallet(os.environ['WALLET_DB_PATH'])

account = wallet.get_account('Alice')

# Sync account with the node
balance = account.sync()

# We try to destroy the first alias in the account
alias_id = balance.aliases[0]

if 'STRONGHOLD_PASSWORD' not in os.environ:
    raise Exception(".env STRONGHOLD_PASSWORD is undefined, see .env.example")

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

# Send transaction.
transaction = account.prepare_destroy_alias(alias_id).send()
print(f'Block sent: {os.environ["EXPLORER_URL"]}/block/{transaction.blockId}')
