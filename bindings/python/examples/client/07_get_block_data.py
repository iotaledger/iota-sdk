from iota_sdk import Client
from dotenv import load_dotenv
import json
import os

load_dotenv()

node_url = os.environ.get('NODE_URL', 'https://api.testnet.shimmer.network')

# Create a Client instance
client = Client(nodes=[node_url])

# Fetch a block ID from the node
block_ids = client.get_tips()
print(f'Block id: {block_id[0]}')

# Get the metadata for the block
metadata = client.get_block_metadata(block_id[0])
print(f'Block metadata: {json.dumps(metadata, indent=4)}')

# Request the block by its id
block = client.get_block_data(block_id[0])
print(f'Block: {json.dumps(block, indent=4)}')