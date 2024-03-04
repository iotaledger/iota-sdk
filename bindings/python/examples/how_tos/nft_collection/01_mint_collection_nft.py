import os
import sys

from dotenv import load_dotenv
from iota_sdk import MintNftParams, Utils, Wallet, WalletOptions, Irc27Metadata

load_dotenv()

# The NFT collection size
NFT_COLLECTION_SIZE = 150
# Mint NFTs in chunks since the transaction size is limited
NUM_NFTS_MINTED_PER_TRANSACTION = 50

# In this example we will mint some collection NFTs with issuer feature.

if len(sys.argv) < 2:
    raise Exception("missing example argument: ISSUER_NFT_ID")

issuer_nft_id = sys.argv[1]

wallet = Wallet(WalletOptions(storage_path=os.environ.get('WALLET_DB_PATH')))

if 'STRONGHOLD_PASSWORD' not in os.environ:
    raise Exception(".env STRONGHOLD_PASSWORD is undefined, see .env.example")

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

# Sync wallet with the node
wallet.sync()

bech32_hrp = wallet.get_client().get_bech32_hrp()
issuer = Utils.nft_id_to_bech32(issuer_nft_id, bech32_hrp)


def get_immutable_metadata(index: int) -> str:
    """Returns the immutable metadata for the NFT with the given index"""
    Irc27Metadata(
        "video/mp4",
        "https://ipfs.io/ipfs/QmPoYcVm9fx47YXNTkhpMEYSxCD3Bqh7PJYr7eo5YjLgiT",
        "Shimmer OG NFT #" + str(index),
        description="The Shimmer OG NFT was handed out 1337 times by the IOTA Foundation to celebrate the official launch of the Shimmer Network.",
        issuerName="IOTA Foundation",
        collectionName="Shimmer OG",
    ).as_hex()


# Create the metadata with another index for each
nft_mint_params = list(
    map(
        lambda index: MintNftParams(
            immutable_metadata=get_immutable_metadata(index), issuer=issuer
        ),
        range(NFT_COLLECTION_SIZE),
    )
)

while nft_mint_params:
    chunk, nft_mint_params = (
        nft_mint_params[:NUM_NFTS_MINTED_PER_TRANSACTION],
        nft_mint_params[NUM_NFTS_MINTED_PER_TRANSACTION:],
    )
    print(
        f'Minting {len(chunk)} NFTs... ({NFT_COLLECTION_SIZE-len(nft_mint_params)}/{NFT_COLLECTION_SIZE})'
    )
    transaction = wallet.mint_nfts(chunk)

    wallet.wait_for_transaction_acceptance(
        transaction.transaction_id)

    print(
        f'Tx accepted: {os.environ["EXPLORER_URL"]}/transactions/{transaction.transaction_id}')

    # Sync so the new outputs are available again for new transactions
    wallet.sync()
