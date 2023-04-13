from iota_client import *
import json

# Create an IotaClient instance
client = IotaClient(nodes = ['https://api.testnet.shimmer.network'])

hex_address = client.bech32_to_hex('rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy')

address_unlock_condition = AddressUnlockCondition(
    Ed25519Address(hex_address)
)

# Build most basic output with amound and a single address unlock condition
basic_output = client.build_basic_output(
    unlock_conditions=[
        address_unlock_condition
    ],
    amount=1000000,
)
print(json.dumps(basic_output, indent=4))

# Output with metadata feature block
basic_output = client.build_basic_output(
    unlock_conditions=[
        address_unlock_condition,
    ],
    features=[
        MetadataFeature('0x'+'Hello, World!'.encode('utf-8').hex())
    ],
    amount=1000000,
)
print(json.dumps(basic_output, indent=4))

# Output with storage deposit return
basic_output = client.build_basic_output(
    unlock_conditions=[
        address_unlock_condition,
        StorageDepositReturnUnlockCondition(
            return_address=Ed25519Address(hex_address),
            amount=1000000
        )
    ],
    amount=1000000,
)
print(json.dumps(basic_output, indent=4))

# Output with expiration
basic_output = client.build_basic_output(
    unlock_conditions=[
        address_unlock_condition,
        ExpirationUnlockCondition(
            return_address=Ed25519Address(hex_address),
            unix_time=1
        )
    ],
    amount=1000000,
)
print(json.dumps(basic_output, indent=4))

# Output with timelock
basic_output = client.build_basic_output(
    unlock_conditions=[
        address_unlock_condition,
        TimelockUnlockCondition(
            unix_time=1
        )
    ],
    amount=1000000,
)
print(json.dumps(basic_output, indent=4))
