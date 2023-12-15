import os
from dotenv import load_dotenv
from iota_sdk import MnemonicSecretManager, CoinType, SecretManager, Client, TaggedDataPayload, utf8_to_hex, Bip44

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

# Create and post a block without payload
# TODO: have a way in the bindings to send an empty block
# https://github.com/iotaledger/iota-sdk/issues/647
unsigned_block = client.build_basic_block(
    issuer_id,
    TaggedDataPayload(
        utf8_to_hex("tag"),
        utf8_to_hex("data")))[0]
block = secret_manager.sign_block(unsigned_block, chain)
block_id = client.post_block(block)

print(f'Empty block sent: {os.environ["EXPLORER_URL"]}/block/{block_id}')
