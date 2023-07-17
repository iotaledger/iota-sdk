from iota_sdk import Wallet, Utils, utf8_to_hex, MintNftParams
from dotenv import load_dotenv
import os

load_dotenv()

# In this example we will mint the issuer NFT for the NFT collection.

wallet = Wallet(os.environ['WALLET_DB_PATH'])

if 'STRONGHOLD_PASSWORD' not in os.environ:
    raise Exception(".env STRONGHOLD_PASSWORD is undefined, see .env.example")

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

account = wallet.get_account('Alice')

# Sync account with the node
account.sync()

# Issue the minting transaction and wait for its inclusion
print('Sending NFT minting transaction...')
params = MintNftParams(
    immutableMetadata=utf8_to_hex(
        "This NFT will be the issuer from the awesome NFT collection"),
)


prepared = account.prepare_mint_nfts([params])
transaction = prepared.send()

# Wait for transaction to get included
block_id = account.retry_transaction_until_included(transaction.transactionId)

print(
    f'Block sent: {os.environ["EXPLORER_URL"]}/block/{block_id}')

essence = transaction.payload["essence"]

for outputIndex, output in enumerate(essence["outputs"]):
    # New minted NFT id is empty in the output
    if output["type"] == 6 and output["nftId"] == '0x0000000000000000000000000000000000000000000000000000000000000000':
        outputId = Utils.compute_output_id(
            transaction.transactionId, outputIndex)
        nftId = Utils.compute_nft_id(outputId)
        print(f'New minted NFT id: {nftId}')
