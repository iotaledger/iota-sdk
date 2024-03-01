import dataclasses
import json
import os

from dotenv import load_dotenv
from iota_sdk import Client

load_dotenv()

node_url = os.environ.get('NODE_URL', 'https://api.testnet.shimmer.network')

# Create a Client instance
client = Client(nodes=[node_url])

list_index = 1
slot_index = client.get_node_info().info.status.latest_finalized_slot
cursor = str(slot_index) + "," + str(list_index)

validators_response = client.get_validators(1, cursor)
print(f'{json.dumps(dataclasses.asdict(validators_response), indent=4)}')
