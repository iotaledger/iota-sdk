import os

from dotenv import load_dotenv

from iota_sdk import SecretManager, StrongholdSecretManager

load_dotenv()

for env_var in ['MNEMONIC', 'STRONGHOLD_PASSWORD']:
    if env_var not in os.environ:
        raise Exception(f'.env {env_var} is undefined, see .env.example')


secret_manager = SecretManager(StrongholdSecretManager(
    "example.stronghold", os.environ['STRONGHOLD_PASSWORD']))

# Store the mnemonic in the Stronghold snapshot, this needs to be done only the first time.
# The mnemonic can't be retrieved from the Stronghold file, so make a
# backup in a secure place!
secret_manager.store_mnemonic(
    os.environ['MNEMONIC'])

# Generate public address with custom account index and range.
address = secret_manager.generate_ed25519_addresses(
    account_index=0, start=0, end=1)

print(f'Address: {address[0]}')
