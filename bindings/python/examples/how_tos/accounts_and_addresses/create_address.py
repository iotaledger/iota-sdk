import os

from dotenv import load_dotenv

from iota_sdk import StrongholdSecretManager, SecretManager

load_dotenv()

# This example generates a new address.

if 'STRONGHOLD_PASSWORD' not in os.environ:
    raise Exception(".env STRONGHOLD_PASSWORD is undefined, see .env.example")

secret_manager = SecretManager(StrongholdSecretManager(
    os.environ.get('STRONGHOLD_SNAPSHOT_PATH'),
    os.environ.get('STRONGHOLD_PASSWORD')
))

address = secret_manager.generate_ed25519_addresses(1)
print('Generated address:', address[0].address)
