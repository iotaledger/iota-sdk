import os

from dotenv import load_dotenv
from iota_sdk import Wallet, WalletOptions

load_dotenv()

# This example gets a client from the wallet.

wallet = Wallet(WalletOptions(storage_path=os.environ.get('WALLET_DB_PATH')))

client = wallet.get_client()

node_info = client.get_node_info()
print(f'{node_info}')
