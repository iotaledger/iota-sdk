import json
import os

from dotenv import load_dotenv

from iota_sdk import (AddressUnlockCondition, Client, Ed25519Address,
                      ExpirationUnlockCondition, MetadataFeature,
                      SenderFeature, StorageDepositReturnUnlockCondition,
                      TagFeature, TimelockUnlockCondition, Utils, utf8_to_hex)

load_dotenv()

node_url = os.environ.get('NODE_URL', 'https://api.testnet.shimmer.network')

# Create a Client instance
client = Client(nodes=[node_url])

hex_address = Utils.bech32_to_hex(
    'rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy')

address_unlock_condition = AddressUnlockCondition(
    Ed25519Address(hex_address)
)

# Build most basic output with amount and a single address unlock condition
basic_output = client.build_basic_output(
    unlock_conditions=[
        address_unlock_condition
    ],
    amount=1000000,
)
print(json.dumps(basic_output.as_dict(), indent=4))

# Output with metadata feature block
basic_output = client.build_basic_output(
    unlock_conditions=[
        address_unlock_condition,
    ],
    features=[
        MetadataFeature(utf8_to_hex('Hello, World!'))
    ],
    amount=1000000,
)
print(json.dumps(basic_output.as_dict(), indent=4))

# Output with storage deposit return
basic_output = client.build_basic_output(
    unlock_conditions=[
        address_unlock_condition,
        StorageDepositReturnUnlockCondition(
            returnAddress=Ed25519Address(hex_address),
            amount=1000000
        )
    ],
    amount=1000000,
)
print(json.dumps(basic_output.as_dict(), indent=4))

# Output with expiration
basic_output = client.build_basic_output(
    unlock_conditions=[
        address_unlock_condition,
        ExpirationUnlockCondition(
            returnAddress=Ed25519Address(hex_address),
            unixTime=1
        )
    ],
    amount=1000000,
)
print(json.dumps(basic_output.as_dict(), indent=4))

# Output with timelock
basic_output = client.build_basic_output(
    unlock_conditions=[
        address_unlock_condition,
        TimelockUnlockCondition(1)
    ],
    amount=1000000,
)
print(json.dumps(basic_output.as_dict(), indent=4))

# Output with tag feature
basic_output = client.build_basic_output(
    unlock_conditions=[
        address_unlock_condition
    ],
    features=[
        TagFeature(utf8_to_hex('Hello, World!'))
    ],
    amount=1000000,
)
print(json.dumps(basic_output.as_dict(), indent=4))

# Output with sender feature
basic_output = client.build_basic_output(
    unlock_conditions=[
        address_unlock_condition
    ],
    features=[
        SenderFeature(Ed25519Address(hex_address))
    ],
    amount=1000000,
)
print(json.dumps(basic_output.as_dict(), indent=4))
