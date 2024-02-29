import os

from dotenv import load_dotenv
from iota_sdk import SendNftParams, Wallet, WalletOptions

load_dotenv()

# In this example we will send an nft

wallet = Wallet(WalletOptions(storage_path=os.environ.get('WALLET_DB_PATH')))

if 'STRONGHOLD_PASSWORD' not in os.environ:
    raise Exception(".env STRONGHOLD_PASSWORD is undefined, see .env.example")

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

# Sync wallet with the node
balance = wallet.sync()

outputs = [SendNftParams(
    address="rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu",
    nft_id=balance.nfts[0],
)]

transaction = wallet.send_nft(outputs)
print(f'Block sent: {os.environ["EXPLORER_URL"]}/block/{transaction.block_id}')
