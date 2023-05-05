from iota_sdk import *
from dotenv import load_dotenv
import os

load_dotenv()

node_url = os.environ.get('NODE_URL', 'https://api.testnet.shimmer.network')

# Create a Client instance
client = Client(nodes=[node_url])

if 'NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1' not in os.environ:
    print(".env mnemonic is undefined, see .env.example")
    sys.exit(1)

secret_manager = MnemonicSecretManager(os.environ['NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1'])

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
        MetadataFeature(utf8_to_hex('Hello, World!'))
    ],
    features=[
        MetadataFeature(utf8_to_hex('Hello, World!'))
    ]
)

# Create and post a block with the nft output
block = client.build_and_post_block(secret_manager, outputs=[nft_output])
print(f'NFT mint block sent: {os.environ["EXPLORER_URL"]}/block/{block[0]}')
