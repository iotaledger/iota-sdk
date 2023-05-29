from iota_sdk import Wallet, HexStr
from dotenv import load_dotenv
import os

load_dotenv()

# In this example we will decrease the native token supply

wallet = Wallet('./alice-database')

account = wallet.get_account('Alice')

# Sync account with the node
account.sync()

if 'STRONGHOLD_PASSWORD' not in os.environ:
    raise Exception(".env STRONGHOLD_PASSWORD is undefined, see .env.example")

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

# TODO: replace with your own values.
token_id = HexStr('0x08429fe5864378ce70699fc2d22bb144cb86a3c4833d136e3b95c5dadfd6ba0cef0500000000')
mint_amount = 10

# Prepare and send transaction.
prepared_transaction = account.prepare_increase_native_token_supply(token_id, mint_amount)
transaction = prepared_transaction.send()

print(f'Block sent: {os.environ["EXPLORER_URL"]}/block/{transaction["blockId"]}')
