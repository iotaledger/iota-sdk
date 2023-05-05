from iota_sdk import Client
from dotenv import load_dotenv

import json
import os

load_dotenv()

node_url = os.environ.get('NODE_URL', 'https://api.testnet.shimmer.network')

# Create a Client instance
client = Client(nodes=[node_url])

# Get an outputs by its id
output = client.get_output('0x1e857d380f813d8035e487b6dfd2ff4740b6775273ba1b576f01381ba2a1a44c0000')
print(json.dumps(output, indent=4))
