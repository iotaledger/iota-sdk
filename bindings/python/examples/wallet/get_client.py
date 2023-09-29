import os

from dotenv import load_dotenv

from iota_sdk import Wallet

load_dotenv()

# This example gets a client from the wallet.

wallet = Wallet(os.environ['WALLET_DB_PATH'])

client = wallet.get_client()

info = client.get_info()
print(f'{info}')
