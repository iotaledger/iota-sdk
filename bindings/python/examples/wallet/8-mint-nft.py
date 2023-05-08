from iota_sdk import Wallet, utf8_to_hex
from dotenv import load_dotenv
import os

load_dotenv()

# In this example we will mint an nft

wallet = Wallet('./alice-database')

account = wallet.get_account('Alice')

# Sync account with the node
response = account.sync()

if 'STRONGHOLD_PASSWORD' not in os.environ:
    print(".env STRONGHOLD_PASSWORD is undefined, see .env.example")
    sys.exit(1)

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

outputs = [{
    "immutableMetadata": utf8_to_hex("some immutable nft metadata"),
}]

transaction = account.mint_nfts(outputs)

print(f'Block sent: {os.environ["EXPLORER_URL"]}/block/{transaction["blockId"]}')
