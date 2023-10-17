import json
import os

from dotenv import load_dotenv

from iota_sdk import OutputParams, Unlocks, Wallet

load_dotenv()

# In this example we will prepare an output with an address and expiration
# unlock condition and send it.

wallet = Wallet(os.environ['WALLET_DB_PATH'])

account = wallet.get_account("Alice")

if 'STRONGHOLD_PASSWORD' not in os.environ:
    raise Exception(".env STRONGHOLD_PASSWORD is undefined, see .env.example")

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

# using prepare_output
output = account.prepare_output(
    OutputParams(
        "rms1qprutadk4uc9rp6h7wh7sech42sl0z40ztpgyygr5tf0cn5jrqshgm8y43d",
        1000000,
        unlocks=Unlocks(
            expiration_slot_index=1676570528)))
print(f"Output: {json.dumps(output.to_dict(), indent=4)}")

account.sync()

transaction = account.send_outputs([output])
print(f'Block sent: {os.environ["EXPLORER_URL"]}/block/{transaction.block_id}')
