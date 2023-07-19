from iota_sdk import Wallet
from dotenv import load_dotenv
import os

# This example requests funds from the faucet

load_dotenv()

FAUCET_URL = os.environ.get(
    'FAUCET_URL',
    'https://faucet.testnet.shimmer.network/api/enqueue')

wallet = Wallet(os.environ['WALLET_DB_PATH'])

account = wallet.get_account('Alice')

address = account.addresses()[0].address
print(address)

response = wallet.get_client().request_funds_from_faucet(FAUCET_URL, address=address)
print(response)
