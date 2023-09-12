import json
import os

from dotenv import load_dotenv

from iota_sdk import (AliasAddress, Client,
                      ImmutableAliasAddressUnlockCondition, SimpleTokenScheme)

load_dotenv()

node_url = os.environ.get('NODE_URL', 'https://api.testnet.shimmer.network')

# Create a Client instance
client = Client(nodes=[node_url])

# Configure foundry output
# Replace with your own values
serial_number = 1
token_scheme = SimpleTokenScheme(32, 0, 64)
unlock_conditions = [
    ImmutableAccountAddressUnlockCondition(
        AccountAddress(
            '0xa5c28d5baa951de05e375fb19134ea51a918f03acc2d0cee011a42b298d3effa')
    )
]

# Configure and build and foundry output
output = client.build_foundry_output(
    1,
    token_scheme,
    unlock_conditions,
)

# Print the output
print(json.dumps(output.to_dict(), indent=4))
