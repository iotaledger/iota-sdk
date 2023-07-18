from iota_sdk import Wallet
from dotenv import load_dotenv
import os

# This example gets a client from the wallet.

wallet = Wallet(os.environ['WALLET_DB_PATH'])

client = wallet.get_client()

info = client.get_info()
print(f'{info}')
