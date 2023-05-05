from iota_sdk import Wallet
from dotenv import load_dotenv
import json
import os

load_dotenv()

# In this example we will prepare an output with an address and expiration unlock condition and send it

wallet = Wallet("./alice-database")

account = wallet.get_account("Alice")

if 'STRONGHOLD_PASSWORD' not in os.environ:
    print(".env STRONGHOLD_PASSWORD is undefined, see .env.example")
    sys.exit(1)

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

# using prepare_output
output = account.prepare_output(
    {
        "amount": "1000000",
        "recipientAddress": "rms1qprutadk4uc9rp6h7wh7sech42sl0z40ztpgyygr5tf0cn5jrqshgm8y43d",
        "unlocks":
            {
                "expirationUnixTime": 1676570528,
            },
    }
)
print(f"Output: {json.dumps(output, indent=4)}")

account.sync()

transaction = account.send_outputs([output])
print(f'Block sent: {os.environ["EXPLORER_URL"]}/block/{transaction["blockId"]}')
