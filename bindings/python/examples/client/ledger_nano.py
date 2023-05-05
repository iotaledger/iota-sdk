from iota_sdk import Client, LedgerNanoSecretManager, SecretManager
from dotenv import load_dotenv
import os

load_dotenv()

# In this example we will get the ledger status and generate an address
# To use the ledger nano simulator clone https://github.com/iotaledger/ledger-shimmer-app, run `git submodule init && git submodule update --recursive`,
# then `./build.sh -m nanos|nanox|nanosplus -s` and use `True` in `LedgerNanoSecretManager(True)`.

node_url = os.environ.get('NODE_URL', 'https://api.testnet.shimmer.network')

# Create a Client instance
client = Client(nodes=[node_url])

is_simulator = True

ledger_secret_manager = LedgerNanoSecretManager(is_simulator)
secret_manager = SecretManager(ledger_secret_manager)

# Get the Ledger Nano status.
ledger_nano_status = secret_manager.get_ledger_nano_status()

print(f'Ledger Nano status: {ledger_nano_status}')

# Generate public address with custom account index and range.
address = client.generate_addresses(
    ledger_secret_manager, account_index=0, start=0, end=1)

print(f'Address: {address[0]}')
