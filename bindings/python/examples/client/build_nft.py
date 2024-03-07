import json
import os

from dotenv import load_dotenv
from iota_sdk import (
    AddressUnlockCondition,
    Client,
    IssuerFeature,
    MetadataFeature,
    SenderFeature,
    TagFeature,
    Utils,
    utf8_to_hex,
    Irc27Metadata,
)

load_dotenv()

node_url = os.environ.get('NODE_URL', 'https://api.testnet.shimmer.network')

# Create a Client instance
client = Client(nodes=[node_url])

address = Utils.parse_bech32_address(
    'rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy')

tip_27_immutable_metadata = Irc27Metadata(
    "image/jpeg",
    "https://mywebsite.com/my-nft-files-1.jpeg",
    "My NFT #0001",
)

# Build NFT output
nft_output = client.build_nft_output(
    unlock_conditions=[
        AddressUnlockCondition(address)
    ],
    # NftId needs to be null the first time
    nft_id='0x0000000000000000000000000000000000000000000000000000000000000000',
    immutable_features=[
        IssuerFeature(address),
        tip_27_immutable_metadata.as_feature()
    ],
    features=[
        SenderFeature(address),
        MetadataFeature(utf8_to_hex('mutable metadata')),
        TagFeature(utf8_to_hex('my tag'))
    ]
)

# Print the output
print(json.dumps(nft_output.to_dict(), indent=4))
