from iota_sdk import Wallet
from dotenv import load_dotenv
import os

# This example lists all addresses in the account.

# This example uses secrets in environment variables for simplicity which
# should not be done in production.
load_dotenv()

wallet = Wallet(os.environ['WALLET_DB_PATH'])

account = wallet.get_account('Alice')

addresses = account.addresses()

for address in addresses:
    print(address.address)
