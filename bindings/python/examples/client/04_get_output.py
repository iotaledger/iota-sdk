from iota_sdk import Client
from dotenv import load_dotenv

import json
import os

load_dotenv()

node_url = os.environ.get('NODE_URL', 'https://api.testnet.shimmer.network')

# Create a Client instance
client = Client(nodes=[node_url])

# Get an outputs by its id
output_with_metadata = client.get_output(
    '0x022aefa73dff09b35b21ab5493412b0d354ad07a970a12b71e8087c6f3a7b8660000')
print(json.dumps(output_with_metadata.as_dict(), indent=4))
