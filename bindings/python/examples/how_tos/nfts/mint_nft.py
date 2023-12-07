import os

from dotenv import load_dotenv

from iota_sdk import MintNftParams, Wallet, WalletOptions, utf8_to_hex

load_dotenv()

# In this example we will mint an nft

wallet = Wallet(WalletOptions(storage_path=os.environ.get('WALLET_DB_PATH')))

if 'STRONGHOLD_PASSWORD' not in os.environ:
    raise Exception(".env STRONGHOLD_PASSWORD is undefined, see .env.example")

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

# Sync account with the node
response = wallet.sync()

outputs = [MintNftParams(
    immutable_metadata=utf8_to_hex("some immutable nft metadata"),
)]

transaction = wallet.mint_nfts(outputs)
print(f'Block sent: {os.environ["EXPLORER_URL"]}/block/{transaction.block_id}')
