import os

from dotenv import load_dotenv
from iota_sdk import SendParams, Wallet, WalletOptions

# This example uses secrets in environment variables for simplicity which
# should not be done in production.
load_dotenv()

# This example sends a transaction.

for env_var in ['WALLET_DB_PATH', 'STRONGHOLD_PASSWORD']:
    if env_var not in os.environ:
        raise Exception(f'.env {env_var} is undefined, see .env.example')


wallet = Wallet(WalletOptions(storage_path=os.environ.get('WALLET_DB_PATH')))

# Sync wallet with the node
wallet.sync()

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

params = [SendParams(
    address="rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu",
    amount=1000000,
)]

transaction = wallet.send_with_params(params)
print(f'Block sent: {os.environ["EXPLORER_URL"]}/block/{transaction.block_id}')
