from iota_client import IotaClient
import json

# Create an IotaClient instance
client = IotaClient(nodes=['https://api.testnet.shimmer.network'])

hexAddress = client.bech32_to_hex(
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
        {
            "type": 0,
            "address": {
                "type": 0,
                "pubKeyHash": hexAddress,
            },
        },
    ],
    # NftId needs to be null the first time
    nft_id="0x0000000000000000000000000000000000000000000000000000000000000000",
    immutable_features=[
        {
            # issuer feature
            'type': 1,
            'address': {
                'type': 0,
                'pubKeyHash': hexAddress,
            },
        },
        {
            # metadata feature
            'type': 2,
            'data': '0x'+json.dumps(tip_27_immutable_metadata, separators=(',', ':')).encode("utf-8").hex(),
        },
    ],
    features=[
        {
            # sender feature
            'type': 0,
            'address': {
                'type': 0,
                'pubKeyHash': hexAddress,
            },
        },
        {
            # metadata feature
            'type': 2,
            'data': '0x'+'mutable metadata'.encode("utf-8").hex(),
        },
        {
            # tag feature
            'type': 3,
            'tag': '0x'+'my tag'.encode("utf-8").hex(),
        }
    ]
)

# Print the output
print(json.dumps(nft_output, indent=2))
