from iota_sdk import Client, init_logger
from dotenv import load_dotenv

import json
import os

load_dotenv()
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

nft_id = "0x0000000000000000000000000000000000000000000000000000000000000000"
route = "outputs/nft/" + nft_id

# Get an outputs by its id
output = client.call_plugin_route("api/indexer/v1/", "GET", route, None, None)
print(json.dumps(output, indent=4))
