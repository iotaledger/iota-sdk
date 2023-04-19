from iota_sdk import Client

# Create a Client instance
client = Client(nodes = ['https://api.testnet.shimmer.network'])

# Create and post a block without payload
block = client.build_and_post_block()
print(f'{block}')
