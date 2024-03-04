import os

from dotenv import load_dotenv
from iota_sdk import Wallet, WalletOptions

# This example requests funds from the faucet

load_dotenv()

FAUCET_URL = os.environ.get(
    'FAUCET_URL',
    'https://faucet.testnet.shimmer.network/api/enqueue')

wallet = Wallet(WalletOptions(storage_path=os.environ.get('WALLET_DB_PATH')))

address = wallet.address()
print(address)

response = wallet.get_client().request_funds_from_faucet(FAUCET_URL, address=address)
print(response)
