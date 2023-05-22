from iota_sdk import Client, MnemonicSecretManager, SendAmountParams
from dotenv import load_dotenv
import os

load_dotenv()

node_url = os.environ.get('NODE_URL', 'https://api.testnet.shimmer.network')

# Create a Client instance
client = Client(nodes=[node_url])

if 'NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1' not in os.environ:
    raise Exception(".env mnemonic is undefined, see .env.example")

secret_manager = MnemonicSecretManager(
    os.environ['NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1'])

output = SendAmountParams(
    address='rms1qzpf0tzpf8yqej5zyhjl9k3km7y6j0xjnxxh7m2g3jtj2z5grej67sl6l46',
    amount=1000000,
)

# Create and post a block with a transaction
block = client.build_and_post_block(secret_manager, output=output)
print(f'Block sent: {os.environ["EXPLORER_URL"]}/block/{block[0]}')
