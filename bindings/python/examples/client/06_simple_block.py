from iota_sdk import Client
from dotenv import load_dotenv
import os

load_dotenv()

node_url = os.environ.get('NODE_URL', 'https://api.testnet.shimmer.network')

# Create a Client instance
client = Client(nodes=[node_url])

# Create and post a block without payload
block = client.build_and_post_block()
print(f'Empty block sent: {os.environ["EXPLORER_URL"]}/block/{block[0]}')