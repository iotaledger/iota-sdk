from iota_sdk import *
import json

# Create a Client instance
client = Client(nodes=['https://api.testnet.shimmer.network'])

hexAddress = Utils.bech32_to_hex(
    'rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy')

alias_id = '0x0000000000000000000000000000000000000000000000000000000000000000'
state_metadata = data = utf8_to_hex('Hello, World!')
unlock_conditions = [
    StateControllerAddressUnlockCondition(Ed25519Address(hexAddress)),
    GovernorAddressUnlockCondition(Ed25519Address(hexAddress))
]
features = [
    SenderFeature(Ed25519Address(hexAddress)),
    MetadataFeature(utf8_to_hex('Hello, World!'))
]
immutable_features = [
    IssuerFeature(Ed25519Address(hexAddress)),
    MetadataFeature(utf8_to_hex('Hello, World!'))
]

# Build alias output
alias_output = client.build_alias_output(
    alias_id=alias_id,
    state_metadata=state_metadata,
    unlock_conditions=unlock_conditions,
    features=features,
    immutable_features=immutable_features
)

# Print the output
print(json.dumps(alias_output, indent=4))
