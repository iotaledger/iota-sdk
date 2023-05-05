from iota_sdk import Wallet
from dotenv import load_dotenv
import os

load_dotenv()

# In this example we will claim outputs that have additional unlock conditions as expiration or storage deposit return

wallet = Wallet('./alice-database')

account = wallet.get_account('Alice')

if 'STRONGHOLD_PASSWORD' not in os.environ:
    print(".env STRONGHOLD_PASSWORD is undefined, see .env.example")
    sys.exit(1)

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

# Sync account with the node
response = account.sync()

# Only the unspent outputs in the account
output_ids = account.get_outputs_with_additional_unlock_conditions('All')

print(f'Available outputs to claim: {output_ids}')

transaction = account.claim_outputs(output_ids)
print(f'Block sent: {os.environ["EXPLORER_URL"]}/block/{transaction["blockId"]}')

