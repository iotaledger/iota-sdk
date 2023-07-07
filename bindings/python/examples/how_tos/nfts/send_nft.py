from iota_sdk import Wallet
from dotenv import load_dotenv
import os

load_dotenv()

# In this example we will send an nft

wallet = Wallet(os.environ['WALLET_DB_PATH'])

if 'STRONGHOLD_PASSWORD' not in os.environ:
    raise Exception(".env STRONGHOLD_PASSWORD is undefined, see .env.example")

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

account = wallet.get_account('Alice')

# Sync account with the node
response = account.sync()

outputs = [{
    "address": "rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu",
    "nftId": "0x17f97185f80fa56eab974de6b7bbb80fa812d4e8e37090d166a0a41da129cebc",
}]

transaction = account.prepare_send_nft(outputs).send()
print(f'Block sent: {os.environ["EXPLORER_URL"]}/block/{transaction.blockId}')
