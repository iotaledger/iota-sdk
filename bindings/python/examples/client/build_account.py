import json
import os

from dotenv import load_dotenv

from iota_sdk import (Client, Ed25519Address, AddressUnlockCondition,
                      IssuerFeature, MetadataFeature, SenderFeature,
                      Utils,
                      utf8_to_hex)

load_dotenv()

node_url = os.environ.get('NODE_URL', 'https://api.testnet.shimmer.network')

# Create a Client instance
client = Client(nodes=[node_url])

hexAddress = Utils.bech32_to_hex(
    'rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy')

account_id = '0x0000000000000000000000000000000000000000000000000000000000000000'
unlock_conditions = [
    AddressUnlockCondition(Ed25519Address(hexAddress)),
]
features = [
    SenderFeature(Ed25519Address(hexAddress)),
    MetadataFeature(utf8_to_hex('Hello, World!'))
]
immutable_features = [
    IssuerFeature(Ed25519Address(hexAddress)),
    MetadataFeature(utf8_to_hex('Hello, World!'))
]

# Build account output
account_output = client.build_account_output(
    account_id=account_id,
    unlock_conditions=unlock_conditions,
    features=features,
    immutable_features=immutable_features
)

# Print the output
print(json.dumps(account_output.to_dict(), indent=4))
