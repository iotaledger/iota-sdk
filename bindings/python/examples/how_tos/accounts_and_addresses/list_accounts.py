from iota_sdk import Wallet
from dotenv import load_dotenv
import os

# This example lists all accounts in the wallet.

# This example uses secrets in environment variables for simplicity which
# should not be done in production.
load_dotenv()

wallet = Wallet(os.environ['WALLET_DB_PATH'])

for account in wallet.get_accounts():
    print(account.get_metadata())
