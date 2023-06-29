from iota_sdk import Client, TaggedDataPayload, utf8_to_hex
from dotenv import load_dotenv
import os

load_dotenv()

node_url = os.environ.get('NODE_URL', 'https://api.testnet.shimmer.network')

# Create a Client instance
client = Client(nodes=[node_url])

# Create and post a block without payload
# TODO: have a way in the bindings to send an empty block https://github.com/iotaledger/iota-sdk/issues/647
block = client.submit_payload(TaggedDataPayload(utf8_to_hex("tag"), utf8_to_hex("data")))
print(f'Empty block sent: {os.environ["EXPLORER_URL"]}/block/{block[0]}')
