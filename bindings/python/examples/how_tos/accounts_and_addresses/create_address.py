from iota_sdk import Wallet
from dotenv import load_dotenv
import os

load_dotenv()

# This example generates a new address.

wallet = Wallet(os.environ['WALLET_DB_PATH'])

if 'STRONGHOLD_PASSWORD' not in os.environ:
    raise Exception(".env STRONGHOLD_PASSWORD is undefined, see .env.example")

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

account = wallet.get_account('Alice')

address = account.generate_ed25519_addresses(1)
print(f'Generated address:', address[0].address)
