from iota_sdk import Wallet, Utils, OutputParams
from dotenv import load_dotenv
import os

load_dotenv()

# In this example we check if an output has only an address unlock
# condition and that the address is from the account.
wallet = Wallet(os.environ['WALLET_DB_PATH'])

account = wallet.get_account("Alice")

accountAddresses = account.addresses()

# # # # # # # # # # # # # # # # # # # # # # # # # # # # # #
