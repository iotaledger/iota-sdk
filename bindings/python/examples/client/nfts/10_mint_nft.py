from iota_sdk import *

# Create a Client instance
client = Client(nodes=['https://api.testnet.shimmer.network'])

secret_manager = MnemonicSecretManager('flame fever pig forward exact dash body idea link scrub tennis minute ' +
                                       'surge unaware prosper over waste kitten ceiling human knife arch situate civil')

nft_output = client.build_nft_output(
    unlock_conditions=[
        AddressUnlockCondition(
            Ed25519Address(Utils.bech32_to_hex(
                'rms1qzpf0tzpf8yqej5zyhjl9k3km7y6j0xjnxxh7m2g3jtj2z5grej67sl6l46')),
        )
    ],
    nft_id='0x0000000000000000000000000000000000000000000000000000000000000000',
    amount=1000000,
    immutable_features=[
        MetadataFeature(utf8_to_hex('Hello, World!'))
    ],
    features=[
        MetadataFeature(utf8_to_hex('Hello, World!'))
    ]
)

# Create and post a block with the nft output
block = client.build_and_post_block(secret_manager, outputs=[nft_output])
print(dumps(block, indent=4))
