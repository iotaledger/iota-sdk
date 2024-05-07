import os

from dotenv import load_dotenv
from iota_sdk import MintNftParams, Utils, Wallet, WalletOptions, utf8_to_hex, MetadataFeature

load_dotenv()

# In this example we will mint the issuer NFT for the NFT collection.

wallet = Wallet(WalletOptions(storage_path=os.environ.get('WALLET_DB_PATH')))

if 'STRONGHOLD_PASSWORD' not in os.environ:
    raise Exception(".env STRONGHOLD_PASSWORD is undefined, see .env.example")

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

# Sync wallet with the node
wallet.sync()

# Issue the minting transaction and wait for its inclusion
print('Sending NFT minting transaction...')
params = MintNftParams(
    immutable_metadata=MetadataFeature({'data': utf8_to_hex(
        "This NFT will be the issuer from the awesome NFT collection")}),
)


tx = wallet.mint_nfts([params])

wallet.wait_for_transaction_acceptance(
    tx.transaction_id)

print(
    f'Tx accepted: {os.environ["EXPLORER_URL"]}/transactions/{tx.transaction_id}')

transaction = tx.payload.transaction

for outputIndex, output in enumerate(transaction.outputs):
    # New minted NFT id is empty in the output
    if output["type"] == 6 and output["nftId"] == '0x0000000000000000000000000000000000000000000000000000000000000000':
        outputId = Utils.compute_output_id(
            tx.transaction_id, outputIndex)
        nftId = Utils.compute_nft_id(outputId)
        print(f'New minted NFT id: {nftId}')
