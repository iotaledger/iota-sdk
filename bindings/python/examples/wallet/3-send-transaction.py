from iota_sdk import Wallet
from dotenv import load_dotenv
import os

load_dotenv()

# This example sends a transaction.

wallet = Wallet('./alice-database')

account = wallet.get_account('Alice')

# Sync account with the node
response = account.sync()

if 'STRONGHOLD_PASSWORD' not in os.environ:
    print(".env STRONGHOLD_PASSWORD is undefined, see .env.example")
    sys.exit(1)

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

outputs = [{
    "address": "rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu",
    "amount": "1000000",
}]

transaction = account.send_amount(outputs)
print(f'Block sent: {os.environ["EXPLORER_URL"]}/block/{transaction["blockId"]}')