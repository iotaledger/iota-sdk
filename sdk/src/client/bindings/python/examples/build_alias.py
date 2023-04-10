from iota_client import *
import json

# Create an IotaClient instance
client = IotaClient(nodes = ['https://api.testnet.shimmer.network'])

hexAddress = client.bech32_to_hex(
    'rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy')

alias_id = '0x0000000000000000000000000000000000000000000000000000000000000000'
state_metadata = data='0x'+'Hello, World!'.encode('utf-8').hex()
unlock_conditions = [
    UnlockCondition(UnlockConditionType.StateControllerAddress, Ed25519Address(hexAddress)),
    UnlockCondition(UnlockConditionType.GovernorAddress, Ed25519Address(hexAddress))
]
features = [
    Feature(FeatureType.Sender, Ed25519Address(hexAddress)),
    Feature(FeatureType.Metadata, data='0x'+'Hello, World!'.encode('utf-8').hex())
]
immutable_features = [
    Feature(FeatureType.Issuer, issuer=Ed25519Address(hexAddress)),
    Feature(FeatureType.Metadata, data='0x'+'Hello, World!'.encode('utf-8').hex())
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
