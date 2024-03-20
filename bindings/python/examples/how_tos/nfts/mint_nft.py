import os

from dotenv import load_dotenv
from iota_sdk import MintNftParams, Wallet, WalletOptions, utf8_to_hex, MetadataFeature

# This example uses secrets in environment variables for simplicity which
# should not be done in production.
load_dotenv()

# In this example we will mint an nft

for env_var in ['WALLET_DB_PATH', 'STRONGHOLD_PASSWORD']:
    if env_var not in os.environ:
        raise Exception(f'.env {env_var} is undefined, see .env.example')


wallet = Wallet(WalletOptions(storage_path=os.environ.get('WALLET_DB_PATH')))

# Sync wallet with the node
wallet.sync()

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

outputs = [MintNftParams(
    immutable_metadata=MetadataFeature(
        {'data': utf8_to_hex("some immutable nft metadata")}),
)]

transaction = wallet.mint_nfts(outputs)
print(f'Block sent: {os.environ["EXPLORER_URL"]}/block/{transaction.block_id}')
