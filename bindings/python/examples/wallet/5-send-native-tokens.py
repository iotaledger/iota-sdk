from iota_sdk import Wallet
from dotenv import load_dotenv
import os

load_dotenv()

# In this example we will send native tokens

wallet = Wallet('./alice-database')

account = wallet.get_account('Alice')

# Sync account with the node
response = account.sync()

if 'STRONGHOLD_PASSWORD' not in os.environ:
    print(".env STRONGHOLD_PASSWORD is undefined, see .env.example")
    sys.exit(1)

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

outputs = [{
    "address": "rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu",
    "nativeTokens": [(
        "0x08a5526c4a15558b709340822edf00cb348d8606a27e2e59b00432a0afe8afb74d0100000000",
        # 10 hex encoded
        "0xA"
    )],
}];

transaction = account.send_native_tokens(outputs, None)
print(f'Block sent: {os.environ["EXPLORER_URL"]}/block/{transaction["blockId"]}')
