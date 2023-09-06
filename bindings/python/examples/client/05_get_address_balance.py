from iota_sdk import Client, NodeIndexerAPI
from dotenv import load_dotenv
import os

load_dotenv()

node_url = os.environ.get('NODE_URL', 'https://api.testnet.shimmer.network')

# Create a Client instance
client = Client(nodes=[node_url])

address = 'rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy'
query_parameters = NodeIndexerAPI.QueryParameters(
    address,
    has_expiration=False,
    has_timelock=False,
    has_storage_deposit_return=False
)

# Get output ids of basic outputs that can be controlled by this address
# without further unlock constraints.
output_ids_response = client.basic_output_ids(query_parameters)
print(f'{output_ids_response.items}')

# Get the outputs by their id
outputs = client.get_outputs(output_ids_response.items)
print(f'{outputs}')


# Calculate the total amount and native tokens
total_amount = 0
native_tokens = []
for output_with_metadata in outputs:
    output = output_with_metadata.output
    total_amount += int(output.amount)
    if output.native_tokens:
        native_tokens.append(output.native_tokens)

print(
    f'Outputs controlled by {address} have {total_amount} glow and native tokens: {native_tokens}')
