from iota_sdk import Client

# Create a Client instance
client = Client(nodes=['https://api.testnet.shimmer.network'])

# Create and post a block without payload
block_id = client.build_and_post_block()[0]
print(f'{block_id}')

# Get block raw
block_raw = client.get_block_raw(block_id)

# Print block raw
print(f'{block_raw}')
