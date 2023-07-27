from iota_sdk import Wallet
from dotenv import load_dotenv
import os

load_dotenv()

# In this example we will burn an NFT
wallet = Wallet(os.environ['WALLET_DB_PATH'])

if 'STRONGHOLD_PASSWORD' not in os.environ:
    raise Exception(".env STRONGHOLD_PASSWORD is undefined, see .env.example")

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

account = wallet.get_account('Alice')

# Sync account with the node
balance = account.sync()

nftId = balance.nfts[0]

# Send transaction.
transaction = account.prepare_burn_nft(nftId).send()
print(f'Block sent: {os.environ["EXPLORER_URL"]}/block/{transaction.blockId}')
