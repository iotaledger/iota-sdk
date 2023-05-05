from iota_sdk import Wallet
from dotenv import load_dotenv
import json
import os

load_dotenv()

# This example generates a new address.

wallet = Wallet('./alice-database')

if 'STRONGHOLD_PASSWORD' not in os.environ:
    print(".env STRONGHOLD_PASSWORD is undefined, see .env.example")
    sys.exit(1)

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

account = wallet.get_account('Alice')

address = account.generate_addresses(1)
# address = account.generate_addresses(
#     1, {'internal': True, 'metadata': {'syncing': True, 'network': 'Testnet'}})
print(f'Address: {json.dumps(address, indent=4)}')
