import os

from dotenv import load_dotenv

from iota_sdk import Wallet, WalletOptions

load_dotenv()

if 'WALLET_DB_PATH' not in os.environ:
    raise Exception(".env WALLET_DB_PATH is undefined, see .env.example")

# In this example, we create an implicit account creation address.

wallet = Wallet(WalletOptions(storage_path=os.environ.get('WALLET_DB_PATH')))

# Get the implicit account address
address = wallet.implicit_account_creation_address()
print(f'Fund the following address: {address}')
