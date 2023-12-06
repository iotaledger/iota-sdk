import os

from dotenv import load_dotenv

from iota_sdk import Wallet, SendParams

load_dotenv()

# In this example we will send an amount below the minimum amount

wallet = Wallet(os.environ['WALLET_DB_PATH'])

# Sync account with the node
response = wallet.sync()

if 'STRONGHOLD_PASSWORD' not in os.environ:
    raise Exception(".env STRONGHOLD_PASSWORD is undefined, see .env.example")

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

params = [SendParams(
    address="rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu",
    amount=1,
)]

transaction = account.send_with_params(params, {"allowMicroAmount": True})
print(f'Transaction sent: {transaction.transaction_id}')

block_id = account.reissue_transaction_until_included(
    transaction.transaction_id)

print(
    f'Block sent: {os.environ["EXPLORER_URL"]}/block/{block_id}')
