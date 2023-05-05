from iota_sdk import Wallet
from dotenv import load_dotenv
import os

load_dotenv()

# In this example we will create an alias ouput

wallet = Wallet('./alice-database')

account = wallet.get_account('Alice')

# Sync account with the node
account.sync()

if 'STRONGHOLD_PASSWORD' not in os.environ:
    print(".env STRONGHOLD_PASSWORD is undefined, see .env.example")
    sys.exit(1)

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

# Send transaction.
transaction = account.create_alias_output(None, None)
print(f'Block sent: {os.environ["EXPLORER_URL"]}/block/{transaction["blockId"]}')
