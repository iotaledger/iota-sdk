from iota_sdk import Wallet
from dotenv import load_dotenv
import os

load_dotenv()

# In this example we will create an alias ouput

wallet = Wallet(os.environ['WALLET_DB_PATH'])

account = wallet.get_account('Alice')

# Sync account with the node
account.sync()

if 'STRONGHOLD_PASSWORD' not in os.environ:
    raise Exception(".env STRONGHOLD_PASSWORD is undefined, see .env.example")

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

# Send transaction.
transaction = account.prepare_create_alias_output(None, None).send()
print(f'Block sent: {os.environ["EXPLORER_URL"]}/block/{transaction.blockId}')
