from iota_client import IotaClient, NodeIndexerAPI

query_parameters = NodeIndexerAPI.QueryParameter(
    address='rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy',
    has_expiration=False,
    has_timelock=False,
    has_storage_deposit_return=False
)

# Create an IotaClient instance
client = IotaClient(nodes = ['https://api.testnet.shimmer.network'])

# Get output ids of basic outputs that can be controlled by this address without further unlock constraints
output_ids_response = client.basic_output_ids(query_parameters)
print(f'{output_ids_response}')

# Get the outputs by their id
outputs = client.get_outputs(output_ids_response['items'])
print(f'{outputs}')
