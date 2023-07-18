from iota_sdk import Wallet
from dotenv import load_dotenv
import os

load_dotenv()

# In this example we will send an amount below the minimum storage deposit

wallet = Wallet(os.environ['WALLET_DB_PATH'])

account = wallet.get_account('Alice')

# Sync account with the node
response = account.sync()

if 'STRONGHOLD_PASSWORD' not in os.environ:
    raise Exception(".env STRONGHOLD_PASSWORD is undefined, see .env.example")

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

params = [{
    "address": "rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu",
    "amount": "1",
}]

transaction = account.send_with_params(params, {"allowMicroAmount": True})
print(f'Transaction sent: {transaction.transactionId}')

block_id = account.retry_transaction_until_included(transaction.transactionId)

print(
    f'Block sent: {os.environ["EXPLORER_URL"]}/block/{block_id}')
