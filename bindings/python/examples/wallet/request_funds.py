from iota_sdk import Wallet, Client
from dotenv import load_dotenv
import json
import os
import time

load_dotenv()

# This example requests funds from the faucet

wallet = Wallet('./alice-database')

account = wallet.get_account('Alice')

# Sync account with the node
response = account.sync()

# Balance before funding
balance = account.get_balance()
print(
    f'balance before faucet request: { balance[ "baseCoin" ][ "available" ] }')

FAUCET_URL = os.environ.get('FAUCET_URL', 'https://faucet.testnet.shimmer.network/api/enqueue')
response = Client().request_funds_from_faucet(FAUCET_URL, "rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu")
print(json.dumps(response, indent=4))

time.sleep(20)

# Sync account with the node
response = account.sync()

# Balance after funding
balance = account.get_balance()
print(
    f'balance after faucet request: { balance[ "baseCoin" ][ "available" ] }')
