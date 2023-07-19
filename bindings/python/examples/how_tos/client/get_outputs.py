from iota_sdk import Client, NodeIndexerAPI
from dotenv import load_dotenv
import json
import os

load_dotenv()

node_url = os.environ.get('NODE_URL', 'https://api.testnet.shimmer.network')

# Create a Client instance
client = Client(nodes=[node_url])

query_parameters = NodeIndexerAPI.QueryParameters(
    address='rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy',
    has_expiration=False,
    has_timelock=False,
    has_storage_deposit_return=False
)

# Get output ids of basic outputs that can be controlled by this address
# without further unlock constraints.
output_ids_response = client.basic_output_ids(query_parameters)
print('First output of query:')
print(f'ID: {output_ids_response.items[0]}')

# Get the outputs by their id
outputs_with_metadata = client.get_outputs(output_ids_response.items)
print(f'{json.dumps(outputs_with_metadata[0].as_dict(), indent=4)}')
