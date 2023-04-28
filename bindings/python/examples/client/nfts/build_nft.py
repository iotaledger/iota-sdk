from iota_sdk import *
import json

# Create a Client instance
client = Client(nodes=['https://api.testnet.shimmer.network'])

hexAddress = Utils.bech32_to_hex(
    'rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy')

# IOTA NFT Standard - IRC27: https://github.com/iotaledger/tips/blob/main/tips/TIP-0027/tip-0027.md
tip_27_immutable_metadata = {
    "standard": "IRC27",
    "version": "v1.0",
    "type": "image/jpeg",
    "uri": "https://mywebsite.com/my-nft-files-1.jpeg",
    "name": "My NFT #0001"
}

# Build NFT output
nft_output = client.build_nft_output(
    unlock_conditions=[
        AddressUnlockCondition(Ed25519Address(hexAddress))
    ],
    # NftId needs to be null the first time
    nft_id='0x0000000000000000000000000000000000000000000000000000000000000000',
    immutable_features=[
        IssuerFeature(Ed25519Address(hexAddress)),
        MetadataFeature(utf8_to_hex(json.dumps(
            tip_27_immutable_metadata, separators=(',', ':'))))
    ],
    features=[
        SenderFeature(Ed25519Address(hexAddress)),
        MetadataFeature(utf8_to_hex('mutable metadata')),
        TagFeature(utf8_to_hex('my tag'))
    ]
)

# Print the output
print(json.dumps(nft_output, indent=4))
