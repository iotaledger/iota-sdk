from iota_sdk import Wallet
from dotenv import load_dotenv
import os

load_dotenv()

# In this example we will burn an NFT
wallet = Wallet('./alice-database')

account = wallet.get_account('Alice')

# Sync account with the node
response = account.sync()

if 'STRONGHOLD_PASSWORD' not in os.environ:
    print(".env STRONGHOLD_PASSWORD is undefined, see .env.example")
    sys.exit(1)

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

# TODO: replace with your own values.
nftId = "0xf95f4d5344217a2ba19a6c19a47f97d267edf8c4d76a7b8c08072ad35acbebbe"

# Send transaction.
transaction = account.burn_nft(nftId)
print(f'Block sent: {os.environ["EXPLORER_URL"]}/block/{transaction["blockId"]}')

