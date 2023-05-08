from iota_sdk import Client
from dotenv import load_dotenv
import os

load_dotenv()

node_url = os.environ.get('NODE_URL', 'https://api.testnet.shimmer.network')

# Create a Client instance
client = Client(nodes=[node_url])

# Fetch a block ID from the node
block_id = client.get_tips()[0]
print(f'Block id: {block_id}')

# Get block raw
block_raw = client.get_block_raw(block_id)

# Print block raw
print(f'Block bytes: {block_raw}')
