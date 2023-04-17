from iota_client import IotaClient, MnemonicSecretManager, AddressWithAmount

# Create an IotaClient instance
client = IotaClient(nodes = ['https://api.testnet.shimmer.network'])

secret_manager = MnemonicSecretManager("flame fever pig forward exact dash body idea link scrub tennis minute " +
                                       "surge unaware prosper over waste kitten ceiling human knife arch situate civil")

# Create and post a block with a transaction
block = client.build_and_post_block(secret_manager, output=AddressWithAmount(
    address='rms1qzpf0tzpf8yqej5zyhjl9k3km7y6j0xjnxxh7m2g3jtj2z5grej67sl6l46', amount=1000000))
print(f'{block}')
