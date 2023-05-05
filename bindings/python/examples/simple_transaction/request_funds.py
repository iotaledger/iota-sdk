from iota_sdk import Wallet, Client
from dotenv import load_dotenv
import json
import os
import time

load_dotenv()

# This example requests funds from the faucet

wallet = Wallet('./alice-database')

account = wallet.get_account('Alice')

# Check and use password
if 'STRONGHOLD_PASSWORD' not in os.environ:
    print(".env STRONGHOLD_PASSWORD is undefined, see .env.example")
    sys.exit(1)

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

# Sync account with the node
response = account.sync()

# Balance before funding
balance = account.get_balance()
print(
    f'balance before faucet request: { balance[ "baseCoin" ][ "available" ] }')

addresses = account.generate_addresses(1)

FAUCET_URL = os.environ.get('FAUCET_URL', 'https://faucet.testnet.shimmer.network/api/enqueue')
response = Client().request_funds_from_faucet(FAUCET_URL, address=addresses[0]['address'])
print(json.dumps(response, indent=4))

time.sleep(20)

# Sync account with the node
response = account.sync()

# Balance after funding
balance = account.get_balance()
print(
    f'balance after faucet request: { balance[ "baseCoin" ][ "available" ] }')
