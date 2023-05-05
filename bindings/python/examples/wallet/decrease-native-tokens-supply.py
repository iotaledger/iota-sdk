from iota_sdk import Wallet
from dotenv import load_dotenv
import os

load_dotenv()

# In this example we will decrease the native token supply

wallet = Wallet('./alice-database')

account = wallet.get_account('Alice')

# Sync account with the node
account.sync()

if 'STRONGHOLD_PASSWORD' not in os.environ:
    print(".env STRONGHOLD_PASSWORD is undefined, see .env.example")
    sys.exit(1)

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

# TODO: replace with your own values.
token_id = "0x08429fe5864378ce70699fc2d22bb144cb86a3c4833d136e3b95c5dadfd6ba0cef0500000000"
melt_amount = "0x20"

# Send transaction.
transaction = account.decrease_native_token_supply(token_id, melt_amount)
print(f'Block sent: {os.environ["EXPLORER_URL"]}/block/{transaction["blockId"]}')
