from iota_sdk import Client, MnemonicSecretManager
from dotenv import load_dotenv
import os

load_dotenv()

node_url = os.environ.get('NODE_URL', 'https://api.testnet.shimmer.network')

# Create a Client instance
client = Client(nodes=[node_url])

# Create and post a block without payload
block_id = client.build_and_post_block()[0]
blockBytes = client.get_block_raw(block_id)

# Post raw block
result = client.post_block_raw(blockBytes)

# Print block raw
print(f'Posted raw block: {os.environ["EXPLORER_URL"]}/block/{result}')
