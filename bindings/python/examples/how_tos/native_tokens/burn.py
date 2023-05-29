from iota_sdk import Wallet, HexStr
from dotenv import load_dotenv
import os

load_dotenv()

# In this example we will burn native tokens

wallet = Wallet('./alice-database')

account = wallet.get_account('Alice')

# Sync account with the node
response = account.sync()

if 'STRONGHOLD_PASSWORD' not in os.environ:
    raise Exception(".env STRONGHOLD_PASSWORD is undefined, see .env.example")

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

# TODO: replace with your own values.
token_id = HexStr("0x08429fe5864378ce70699fc2d22bb144cb86a3c4833d136e3b95c5dadfd6ba0cef0300000000")
burn_amount = 5

# Send transaction.
transaction = account.prepare_burn_native_token(token_id, burn_amount).send()
print(f'Block sent: {os.environ["EXPLORER_URL"]}/block/{transaction["blockId"]}')
