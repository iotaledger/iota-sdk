import os

from dotenv import load_dotenv

from iota_sdk import Wallet, WalletOptions

load_dotenv()

for env_var in ['WALLET_DB_PATH', 'STRONGHOLD_PASSWORD', 'EXPLORER_URL']:
    if env_var not in os.environ:
        raise Exception(f".env {env_var} is undefined, see .env.example")

# In this example we will destroy an account output

wallet = Wallet(WalletOptions(storage_path=os.environ.get('WALLET_DB_PATH')))

# Sync account with the node
balance = wallet.sync()

# We try to destroy the first account in the account
account_id = balance.accounts[0]

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

# Send transaction.
transaction = wallet.prepare_destroy_account(account_id).send()
print(f'Block sent: {os.environ["EXPLORER_URL"]}/block/{transaction.block_id}')
