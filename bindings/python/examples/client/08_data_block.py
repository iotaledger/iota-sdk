
import json
import os
from dataclasses import asdict
from dotenv import load_dotenv
from iota_sdk import BasicBlockBody, Bip44, CoinType, Client, utf8_to_hex, hex_to_utf8, TaggedDataPayload, MnemonicSecretManager, SecretManager

load_dotenv()

node_url = os.environ.get('NODE_URL', 'https://api.testnet.shimmer.network')
issuer_id = os.environ.get(
    'ISSUER_ID', '0x0000000000000000000000000000000000000000000000000000000000000000')

if 'MNEMONIC' not in os.environ:
    raise Exception(".env MNEMONIC is undefined, see .env.example")

secret_manager = SecretManager(MnemonicSecretManager(os.environ['MNEMONIC']))

# Create a Client instance
client = Client(nodes=[node_url])

chain = Bip44(CoinType.IOTA)

# Create and post a block with a tagged data payload
unsigned_block = client.build_basic_block(
    issuer_id,
    TaggedDataPayload(
        utf8_to_hex("tag"),
        utf8_to_hex("data")))[0]
block = secret_manager.sign_block(unsigned_block, chain)
block_id = client.post_block(block)

print(f'Data block sent: {os.environ["EXPLORER_URL"]}/block/{block_id}')

block = client.get_block(block_id).block

if isinstance(block, BasicBlockBody):
    print(f'Block data: {json.dumps(asdict(block), indent=4)}')

    payload = block.payload

    if payload and 'data' in payload and payload['data']:
        print(f'Decoded data: { hex_to_utf8(payload["data"]) }')
else:
    raise ValueError("block must be an instance of BasicBlock")
