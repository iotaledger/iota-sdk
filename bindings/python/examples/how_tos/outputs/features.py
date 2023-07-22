from iota_sdk import *
from dotenv import load_dotenv
import json

load_dotenv()

client = Client()

hex_address = Utils.bech32_to_hex(
    'rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy')

address_unlock_condition = AddressUnlockCondition(
    Ed25519Address(hex_address)
)

# Output with sender feature
nft_output = client.build_nft_output(
    nft_id='0x0000000000000000000000000000000000000000000000000000000000000000',
    unlock_conditions=[
        address_unlock_condition
    ],
    features=[
        SenderFeature(Ed25519Address(hex_address))
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
        IssuerFeature(Ed25519Address(hex_address))
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
        MetadataFeature(utf8_to_hex('Hello, World!'))
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
        MetadataFeature(utf8_to_hex('Hello, World!'))
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
