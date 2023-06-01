from iota_sdk import Wallet, utf8_to_hex
from dotenv import load_dotenv
import time
import os

load_dotenv()

# In this example we will mint native tokens

wallet = Wallet('./alice-database')

account = wallet.get_account('Alice')

if 'STRONGHOLD_PASSWORD' not in os.environ:
    raise Exception(".env STRONGHOLD_PASSWORD is undefined, see .env.example")

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

# Sync account with the node
response = account.sync()

transaction = account.prepare_create_alias_output(None, None).send()

# Wait a few seconds for the transaction to get confirmed
time.sleep(7)

account.sync()

params = {
    "circulatingSupply": utf8_to_hex("1000"),
    "maximumSupply":  utf8_to_hex("1000"),
    "foundryMetadata": utf8_to_hex("171"),
}

transaction = account.prepare_mint_native_token(params, None)
print(f'Token id: {transaction.token_id()}')

transaction = transaction.send()
print(f'Block sent: {os.environ["EXPLORER_URL"]}/block/{transaction["blockId"]}')
