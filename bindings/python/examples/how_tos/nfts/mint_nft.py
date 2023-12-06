import os

from dotenv import load_dotenv

from iota_sdk import MintNftParams, Wallet, utf8_to_hex

load_dotenv()

# In this example we will mint an nft

wallet = Wallet(os.environ['WALLET_DB_PATH'])

if 'STRONGHOLD_PASSWORD' not in os.environ:
    raise Exception(".env STRONGHOLD_PASSWORD is undefined, see .env.example")

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

# Sync account with the node
response = wallet.sync()

outputs = [MintNftParams(
    immutable_metadata=utf8_to_hex("some immutable nft metadata"),
)]

transaction = account.mint_nfts(outputs)
print(f'Block sent: {os.environ["EXPLORER_URL"]}/block/{transaction.block_id}')
