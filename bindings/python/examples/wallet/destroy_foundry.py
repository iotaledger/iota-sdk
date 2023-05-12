from bindings.python.iota_sdk.types.burn import Burn
from iota_sdk import Wallet
from dotenv import load_dotenv
import os

load_dotenv()

# In this example we will destroy a foundry

wallet = Wallet("./alice-database")

account = wallet.get_account("Alice")

# Sync account with the node
response = account.sync()

if "STRONGHOLD_PASSWORD" not in os.environ:
    print(".env STRONGHOLD_PASSWORD is undefined, see .env.example")
    sys.exit(1)

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

# TODO: replace with your own values.
foundry_id = (
    "0x08429fe5864378ce70699fc2d22bb144cb86a3c4833d136e3b95c5dadfd6ba0cef0500000000"
)

# Create burn object.
to_burn = Burn().add_foundry(foundry_id)

# Send transaction.
transaction = account.burn(to_burn)
print(f'Block sent: {os.environ["EXPLORER_URL"]}/block/{transaction["blockId"]}')
