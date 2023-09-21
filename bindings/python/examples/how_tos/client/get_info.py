import dataclasses
import json
import os

from dotenv import load_dotenv

from iota_sdk import Client

load_dotenv()

node_url = os.environ.get('NODE_URL', 'https://api.testnet.shimmer.network')

# Create a Client instance
client = Client(nodes=[node_url])

# Get the node info
node_info = client.get_info().node_info
print(f'{json.dumps(dataclasses.asdict(node_info), indent=4)}')
