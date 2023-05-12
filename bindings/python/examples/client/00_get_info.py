from iota_sdk import Client
from dotenv import load_dotenv
import os

load_dotenv()

node_url = os.environ.get('NODE_URL', 'https://api.testnet.shimmer.network')

# Create a Client instance
client = Client(nodes=[node_url])

# Get the node info
node_info = client.get_info()
print(f'{node_info}')
