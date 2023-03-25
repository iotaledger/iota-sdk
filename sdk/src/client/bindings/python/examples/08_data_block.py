from iota_client import IotaClient

# Create an IotaClient instance
client = IotaClient(nodes = ['https://api.testnet.shimmer.network'])

# Create and post a block with a tagged data payload. Data is 'hello' hex encoded
block = client.build_and_post_block(
    None, tag='0x68656c6c6f', data='0x68656c6c6f')
print(f'{block}')
