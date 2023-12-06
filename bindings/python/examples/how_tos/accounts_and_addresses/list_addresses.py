import os

from dotenv import load_dotenv

from iota_sdk import Wallet

# This example lists all addresses in the account.

# This example uses secrets in environment variables for simplicity which
# should not be done in production.
load_dotenv()

wallet = Wallet(os.environ['WALLET_DB_PATH'])

address = wallet.address()

for address in addresses:
    print(address.address)
