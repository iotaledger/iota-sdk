from iota_sdk import Wallet
from dotenv import load_dotenv
import os

load_dotenv()

# In this example we will destroy an alias output

# TODO: replace with your own values.
ALIAS_ID = "0x982667c59ade8ab8a99188f4de38c68b97fc2ca7ba28a1e9d8d683996247e152"

wallet = Wallet('./alice-database')

account = wallet.get_account('Alice')

# Sync account with the node
response = account.sync()

if 'STRONGHOLD_PASSWORD' not in os.environ:
    print(".env STRONGHOLD_PASSWORD is undefined, see .env.example")
    sys.exit(1)

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

# Send transaction.
transaction = account.destroy_alias(ALIAS_ID)
print(f'Block sent: {os.environ["EXPLORER_URL"]}/block/{transaction["blockId"]}')
