from iota_sdk import Wallet, Utils, utf8_to_hex, MintNftParams
from dotenv import load_dotenv
import os
import sys
import json

load_dotenv()

# The NFT collection size
NFT_COLLECTION_SIZE = 150
# Mint NFTs in chunks since the transaction size is limited
NUM_NFTS_MINTED_PER_TRANSACTION = 50

# In this example we will mint some collection NFTs with issuer feature.

if len(sys.argv) < 2:
    raise Exception("missing example argument: ISSUER_NFT_ID")

issuer_nft_id = sys.argv[1]

wallet = Wallet(os.environ['WALLET_DB_PATH'])

if 'STRONGHOLD_PASSWORD' not in os.environ:
    raise Exception(".env STRONGHOLD_PASSWORD is undefined, see .env.example")

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

account = wallet.get_account('Alice')

# Sync account with the node
account.sync()

bech32_hrp = wallet.get_client().get_bech32_hrp()
issuer = Utils.nft_id_to_bech32(issuer_nft_id, bech32_hrp)


def get_immutable_metadata(index: int, issuer_nft_id: str) -> str:
    data = {
        "standard": "IRC27",
        "version": "v1.0",
        "type": "video/mp4",
        "uri": "ipfs://wrongcVm9fx47YXNTkhpMEYSxCD3Bqh7PJYr7eo5Ywrong",
        "name": "Shimmer OG NFT #" + str(index),
        "description": "The Shimmer OG NFT was handed out 1337 times by the IOTA Foundation to celebrate the official launch of the Shimmer Network.",
        "issuerName": "IOTA Foundation",
        "collectionId": issuer_nft_id,
        "collectionName": "Shimmer OG"
    }
    return json.dumps(data, separators=(',', ':'))


# Create the metadata with another index for each
nft_mint_params = list(map(lambda index: MintNftParams(
    immutableMetadata=utf8_to_hex(
        get_immutable_metadata(index, issuer_nft_id)),
    issuer=issuer
), range(NFT_COLLECTION_SIZE)))

while nft_mint_params:
    chunk, nft_mint_params = nft_mint_params[:NUM_NFTS_MINTED_PER_TRANSACTION], nft_mint_params[NUM_NFTS_MINTED_PER_TRANSACTION:]
    print(
        f'Minting {len(chunk)} NFTs... ({NFT_COLLECTION_SIZE-len(nft_mint_params)}/{NFT_COLLECTION_SIZE})')
    prepared = account.prepare_mint_nfts(chunk)
    transaction = prepared.send()

    # Wait for transaction to get included
    block_id = account.retry_transaction_until_included(
        transaction.transactionId)

    print(
        f'Block sent: {os.environ["EXPLORER_URL"]}/block/{block_id}')

    # Sync so the new outputs are available again for new transactions
    account.sync()
