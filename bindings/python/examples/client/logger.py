from iota_sdk import Client, init_logger
import json

# Create the log configuration, the log will be outputted in 'iota.rs.log'
log_config = {
    'name': 'iota.rs.log',
    'levelFilter': 'debug',
    'targetExclusions': ["h2", "hyper", "rustls"]
}

log_config_str = json.dumps(log_config)

init_logger(log_config_str)

# Create a Client instance
client = Client(nodes=['https://api.testnet.shimmer.network'])

# Get the node info
node_info = client.get_info()
print(f'{node_info}')
