import os

from dotenv import load_dotenv
from iota_sdk import Wallet, WalletOptions, SendParams

# This example uses secrets in environment variables for simplicity which
# should not be done in production.
load_dotenv()

# In this example we will send an amount below the minimum amount

for env_var in ['WALLET_DB_PATH', 'STRONGHOLD_PASSWORD']:
    if env_var not in os.environ:
        raise Exception(f'.env {env_var} is undefined, see .env.example')


wallet = Wallet(WalletOptions(storage_path=os.environ.get('WALLET_DB_PATH')))

# Sync wallet with the node
wallet.sync()

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

params = [SendParams(
    address="rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu",
    amount=1,
)]

transaction = wallet.send_with_params(params, {"allowMicroAmount": True})
print(f'Transaction sent: {transaction.transaction_id}')

wallet.wait_for_transaction_acceptance(
    transaction.transaction_id)

print(
    f'Tx accepted: {os.environ["EXPLORER_URL"]}/transactions/{transaction.transaction_id}')
