
import json
import os
from dataclasses import asdict
from bindings.python.iota_sdk.secret_manager.secret_manager import MnemonicSecretManager, SecretManager
from dotenv import load_dotenv
from iota_sdk import BasicBlock, Client, utf8_to_hex, hex_to_utf8, TaggedDataPayload

load_dotenv()

node_url = os.environ.get('NODE_URL', 'https://api.testnet.shimmer.network')
issuer_id = os.environ.get(
    'ISSUER_ID', '0x0000000000000000000000000000000000000000000000000000000000000000')

if 'MNEMONIC' not in os.environ:
    raise Exception(".env MNEMONIC is undefined, see .env.example")

secret_manager = SecretManager(MnemonicSecretManager(os.environ['MNEMONIC']))

# Create a Client instance
client = Client(nodes=[node_url])

# Create and post a block with a tagged data payload
unsigned_block = client.build_basic_block(
    issuer_id,
    TaggedDataPayload(
        utf8_to_hex("tag"),
        utf8_to_hex("data")))[0]
signed_block = secret_manager.sign_block(unsigned_block)
block_id = client.post_block(signed_block)

print(f'Data block sent: {os.environ["EXPLORER_URL"]}/block/{block_id}')

block = client.get_block(block_id).block

if isinstance(block, BasicBlock):
    print(f'Block data: {json.dumps(asdict(block), indent=4)}')

    payload = block.payload

    if payload and 'data' in payload and payload['data']:
        print(f'Decoded data: { hex_to_utf8(payload["data"]) }')
else:
    raise ValueError("block must be an instance of BasicBlock")
