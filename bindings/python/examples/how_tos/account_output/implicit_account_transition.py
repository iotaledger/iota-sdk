import os

from dotenv import load_dotenv
from iota_sdk import Wallet, WalletOptions, SyncOptions

# In this example we transition an implicit account to an account.

load_dotenv()

for env_var in ['WALLET_DB_PATH', 'STRONGHOLD_PASSWORD', 'EXPLORER_URL']:
    if env_var not in os.environ:
        raise Exception(f".env {env_var} is undefined, see .env.example")

# Need to sync the wallet with implicit accounts option enabled.
wallet = Wallet(WalletOptions(storage_path=os.environ.get('WALLET_DB_PATH')))

wallet.sync(SyncOptions(sync_implicit_accounts=True))

implicit_accounts = wallet.implicit_accounts()

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

# Transition to the account output.
transaction = wallet.implicit_account_transition(
    implicit_accounts[0].output_id)
print(f'Transaction sent: {transaction.transaction_id}')

wallet.wait_for_transaction_acceptance(transaction.transaction_id)
print(
    f'Tx accepted: {os.environ["EXPLORER_URL"]}/transactions/{transaction.transaction_id}')
