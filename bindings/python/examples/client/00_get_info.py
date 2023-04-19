from iota_sdk import Client

# Create a Client instance
client = Client(nodes = ['https://api.testnet.shimmer.network'])

# Get the node info
node_info = client.get_info()
print(f'{node_info}')
