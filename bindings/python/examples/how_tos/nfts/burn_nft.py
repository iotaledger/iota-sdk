import os

from dotenv import load_dotenv
from iota_sdk import Wallet, WalletOptions

load_dotenv()

# In this example we will burn an NFT
wallet = Wallet(WalletOptions(storage_path=os.environ.get('WALLET_DB_PATH')))

if 'STRONGHOLD_PASSWORD' not in os.environ:
    raise Exception(".env STRONGHOLD_PASSWORD is undefined, see .env.example")

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

# Sync wallet with the node
balance = wallet.sync()

nftId = balance.nfts[0]

# Send transaction.
transaction = wallet.prepare_burn_nft(nftId).send()
print(f'Block sent: {os.environ["EXPLORER_URL"]}/block/{transaction.block_id}')
