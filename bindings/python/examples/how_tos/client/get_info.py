from iota_sdk import Client
import dataclasses
from dotenv import load_dotenv
import json
import os

load_dotenv()

node_url = os.environ.get('NODE_URL', 'https://api.testnet.shimmer.network')

# Create a Client instance
client = Client(nodes=[node_url])

# Get the node info
node_info = client.get_info().nodeInfo
print(f'{json.dumps(dataclasses.asdict(node_info), indent=4)}')
