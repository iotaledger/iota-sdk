import dataclasses
import json
import os
import sys

from dotenv import load_dotenv
from iota_sdk import Client

load_dotenv()

node_url = os.environ.get('NODE_URL', 'https://api.testnet.shimmer.network')
page_size = 1
cursor = ""

if len(sys.argv) > 1:
    page_size = int(sys.argv[1])
    if len(sys.argv) > 2:
        cursor = sys.argv[2]

# Create a Client instance
client = Client(nodes=[node_url])

validators = client.get_validators(page_size, cursor)
print(f'{json.dumps(dataclasses.asdict(validators), indent=4)}')
