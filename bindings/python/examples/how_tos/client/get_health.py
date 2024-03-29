import os

from dotenv import load_dotenv

from iota_sdk import Client

load_dotenv()

node_url = os.environ.get('NODE_URL', 'https://api.testnet.shimmer.network')

# Create a Client instance
client = Client(nodes=[node_url])

# Get the node health
is_healthy = client.get_health(node_url)
print(f'Healthy: {is_healthy}')
