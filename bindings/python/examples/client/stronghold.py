from iota_sdk import Client, StrongholdSecretManager, SecretManager
from dotenv import load_dotenv
import os

load_dotenv()

node_url = os.environ.get('NODE_URL', 'https://api.testnet.shimmer.network')

# Create a Client instance
client = Client(nodes=[node_url])

if 'NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1' not in os.environ:
    print(".env mnemonic is undefined, see .env.example")
    sys.exit(1)

if 'STRONGHOLD_PASSWORD' not in os.environ:
    print(".env STRONGHOLD_PASSWORD is undefined, see .env.example")
    sys.exit(1)

secret_manager = StrongholdSecretManager(
    "client.stronghold", os.environ['STRONGHOLD_PASSWORD'])

# Store the mnemonic in the Stronghold snapshot, this needs to be done only the first time.
# The mnemonic can't be retrieved from the Stronghold file, so make a backup in a secure place!
result = SecretManager(secret_manager).store_mnemonic(os.environ['NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1'])

# Generate public address with custom account index and range.
address = client.generate_addresses(
    secret_manager, account_index=0, start=0, end=1)

print(f'Address: {address[0]}')
