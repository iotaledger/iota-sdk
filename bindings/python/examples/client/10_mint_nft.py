import os

from dotenv import load_dotenv

from iota_sdk import (AddressUnlockCondition, Client, Ed25519Address,
                      MetadataFeature, MnemonicSecretManager, Utils,
                      utf8_to_hex, Irc27Metadata)

load_dotenv()

node_url = os.environ.get('NODE_URL', 'https://api.testnet.shimmer.network')

# Create a Client instance
client = Client(nodes=[node_url])

if 'MNEMONIC' not in os.environ:
    raise Exception(".env MNEMONIC is undefined, see .env.example")

secret_manager = MnemonicSecretManager(os.environ['MNEMONIC'])

metadata = Irc27Metadata(
    "video/mp4",
    "https://ipfs.io/ipfs/QmPoYcVm9fx47YXNTkhpMEYSxCD3Bqh7PJYr7eo5YjLgiT",
    "Shimmer OG NFT",
    description="The original Shimmer NFT"
)

nft_output = client.build_nft_output(
    unlock_conditions=[
        AddressUnlockCondition(
            Ed25519Address(Utils.bech32_to_hex(
                'rms1qzpf0tzpf8yqej5zyhjl9k3km7y6j0xjnxxh7m2g3jtj2z5grej67sl6l46')),
        )
    ],
    nft_id='0x0000000000000000000000000000000000000000000000000000000000000000',
    amount=1000000,
    immutable_features=[
        metadata.as_feature()
    ],
    features=[
        MetadataFeature(utf8_to_hex('Hello, World!'))
    ]
)

# Create and post a block with the nft output
block = client.build_and_post_block(secret_manager, outputs=[nft_output])
print(f'NFT mint block sent: {os.environ["EXPLORER_URL"]}/block/{block[0]}')
