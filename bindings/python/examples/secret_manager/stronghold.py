from iota_sdk import StrongholdSecretManager, SecretManager
from dotenv import load_dotenv
import os

load_dotenv()

if 'NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1' not in os.environ:
    raise Exception(".env mnemonic is undefined, see .env.example")

if 'STRONGHOLD_PASSWORD' not in os.environ:
    raise Exception(".env STRONGHOLD_PASSWORD is undefined, see .env.example")

secret_manager = SecretManager(StrongholdSecretManager(
    "example.stronghold", os.environ['STRONGHOLD_PASSWORD']))

# Store the mnemonic in the Stronghold snapshot, this needs to be done only the first time.
# The mnemonic can't be retrieved from the Stronghold file, so make a backup in a secure place!
secret_manager.store_mnemonic(
    os.environ['NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1'])

# Generate public address with custom account index and range.
address = secret_manager.generate_ed25519_addresses(account_index=0, start=0, end=1)

print(f'Address: {address[0]}')
