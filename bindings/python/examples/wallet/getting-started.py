# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

import os

from dotenv import load_dotenv

from iota_sdk import (ClientOptions, CoinType, StrongholdSecretManager, Utils,
                      Wallet)

load_dotenv()

# A name to associate with the created account.
ACCOUNT_ALIAS = 'Alice'

# The node to connect to.
node_url = os.environ.get('NODE_URL', 'https://api.testnet.shimmer.network')

# A password to encrypt the stored data.
# WARNING: Never hardcode passwords in production code.
STRONGHOLD_PASSWORD = os.environ.get(
    'STRONGHOLD_PASSWORD', 'a-secure-password')

# The path to store the account snapshot.
STRONGHOLD_SNAPSHOT_PATH = 'vault.stronghold'

# Setup Stronghold secret manager
secret_manager = StrongholdSecretManager(
    STRONGHOLD_SNAPSHOT_PATH, STRONGHOLD_PASSWORD)

# Set up and store the wallet.
client_options = ClientOptions(nodes=[node_url])

wallet = Wallet(
    client_options=client_options,
    coin_type=CoinType.SHIMMER,
    secret_manager=secret_manager
)

# Generate a mnemonic and store it in the Stronghold vault.
# INFO: It is best practice to back up the mnemonic somewhere secure.
mnemonic = Utils.generate_mnemonic()
wallet.store_mnemonic(mnemonic)

# Create an account.
account = wallet.create_account(ACCOUNT_ALIAS)

# Get the first address and print it.
address = account.addresses()[0]
print(f'Address:\n{address.address}')
