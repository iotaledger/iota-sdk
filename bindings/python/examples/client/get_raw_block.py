from iota_sdk import Client

# Create a Client instance
client = Client(nodes=['https://api.testnet.shimmer.network'])

# Fetch a block ID from the node
block_id = client.get_tips()[0]
print(f'{block_id}')

# Get block raw
block_raw = client.get_block_raw(block_id)

# Print block raw
print(f'{block_raw}')
