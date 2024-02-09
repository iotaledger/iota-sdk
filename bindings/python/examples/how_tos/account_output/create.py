import os
import time

from dotenv import load_dotenv

from iota_sdk import Wallet, WalletOptions

# In this example we will create an account output.

load_dotenv()

for env_var in ['WALLET_DB_PATH', 'STRONGHOLD_PASSWORD', 'EXPLORER_URL']:
    if env_var not in os.environ:
        raise Exception(f".env {env_var} is undefined, see .env.example")

wallet = Wallet(WalletOptions(storage_path=os.environ.get('WALLET_DB_PATH')))

# Sync wallet with the node.
balance = wallet.sync()
print(f'Accounts BEFORE: {balance.accounts}')

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

# Send transaction.
transaction = wallet.create_account_output(None, None)
print(f'Block sent: {os.environ["EXPLORER_URL"]}/block/{transaction.block_id}')

time.sleep(10)

balance = wallet.sync()
print(f'Accounts AFTER: {balance.accounts}')
