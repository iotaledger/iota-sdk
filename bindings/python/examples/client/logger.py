import json
import os

from dotenv import load_dotenv

# pylint: disable=no-name-in-module
from iota_sdk import Client, init_logger

load_dotenv()

# Create the log configuration, the log will be outputted in 'client.log'
log_config = {
    'name': 'client.log',
    'levelFilter': 'debug',
    'targetExclusions': ["h2", "hyper", "rustls"]
}

log_config_str = json.dumps(log_config)

init_logger(log_config_str)

node_url = os.environ.get('NODE_URL', 'https://api.testnet.shimmer.network')

# Create a Client instance
client = Client(nodes=[node_url])

# Get the node info
node_info = client.get_info()
print(f'{node_info}')
