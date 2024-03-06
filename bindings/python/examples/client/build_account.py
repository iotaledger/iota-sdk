import json
import os

from dotenv import load_dotenv
from iota_sdk import (Client, AddressUnlockCondition,
                      IssuerFeature, MetadataFeature, SenderFeature,
                      Utils,
                      utf8_to_hex)

load_dotenv()

node_url = os.environ.get('NODE_URL', 'https://api.testnet.shimmer.network')

# Create a Client instance
client = Client(nodes=[node_url])

address = Utils.parse_bech32_address(
    'rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy')

account_id = '0x0000000000000000000000000000000000000000000000000000000000000000'
unlock_conditions = [
    AddressUnlockCondition(address),
]
features = [
    SenderFeature(address),
    MetadataFeature({'data': utf8_to_hex('Hello, World!')})
]
immutable_features = [
    IssuerFeature(address),
    MetadataFeature({'data': utf8_to_hex('Hello, World!')})
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
