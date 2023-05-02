from iota_sdk import Client

# Create a Client instance
client = Client(nodes=['https://api.testnet.shimmer.network'])

# Fetch a block ID from the node
block_id = client.get_tips()[0]
print(f'{block_id}')

# Get the metadata for the block
metadata = client.get_block_metadata(block_id)
print(f'{metadata}')

# Request the block by its id
block = client.get_block_data(block_id)
print(f'{block}')
