
import json
import os
from dataclasses import asdict
from dotenv import load_dotenv
from iota_sdk import Client, utf8_to_hex, hex_to_utf8, TaggedDataPayload

load_dotenv()

node_url = os.environ.get('NODE_URL', 'https://api.testnet.shimmer.network')

# Create a Client instance
client = Client(nodes=[node_url])

# Create and post a block with a tagged data payload
block = client.submit_payload(
    TaggedDataPayload(
        utf8_to_hex("tag"),
        utf8_to_hex("data")))

print(f'Data block sent: {os.environ["EXPLORER_URL"]}/block/{block[0]}')

block = client.get_block_data(block[0])
print(f'Block data: {json.dumps(asdict(block), indent=4)}')

payload = block.payload

if payload and 'data' in payload and payload['data']:
    print(f'Decoded data: { hex_to_utf8(payload["data"]) }')
