import json
from dataclasses import asdict

from dotenv import load_dotenv
from iota_sdk import (
    AddressUnlockCondition,
    Client,
    Utils,
    SenderFeature,
    IssuerFeature,
    MetadataFeature,
    TagFeature,
    utf8_to_hex,
)

load_dotenv()

client = Client()

address = Utils.parse_bech32_address(
    'rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy')

address_unlock_condition = AddressUnlockCondition(address)

# Output with sender feature
nft_output = client.build_nft_output(
    nft_id='0x0000000000000000000000000000000000000000000000000000000000000000',
    unlock_conditions=[
        address_unlock_condition
    ],
    features=[
        SenderFeature(address)
    ],
)
outputs = [nft_output]

# Output with issuer feature
nft_output = client.build_nft_output(
    nft_id='0x0000000000000000000000000000000000000000000000000000000000000000',
    unlock_conditions=[
        address_unlock_condition,
    ],
    immutable_features=[
        IssuerFeature(address)
    ],
)
outputs.append(nft_output)

# Output with metadata feature
nft_output = client.build_nft_output(
    nft_id='0x0000000000000000000000000000000000000000000000000000000000000000',
    unlock_conditions=[
        address_unlock_condition,
    ],
    features=[
        MetadataFeature({'data': utf8_to_hex('Hello, World!')})
    ],
)
outputs.append(nft_output)

# Output with immutable metadata feature
nft_output = client.build_nft_output(
    nft_id='0x0000000000000000000000000000000000000000000000000000000000000000',
    unlock_conditions=[
        address_unlock_condition,
    ],
    immutable_features=[
        MetadataFeature({'data': utf8_to_hex('Hello, World!')})
    ],
)
outputs.append(nft_output)

# Output with tag feature
nft_output = client.build_nft_output(
    nft_id='0x0000000000000000000000000000000000000000000000000000000000000000',
    unlock_conditions=[
        address_unlock_condition
    ],
    features=[
        TagFeature(utf8_to_hex('Hello, World!'))
    ],
)
outputs.append(nft_output)

print(json.dumps([asdict(o) for o in outputs], indent=2))
