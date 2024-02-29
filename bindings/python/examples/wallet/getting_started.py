# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

import os

from dotenv import load_dotenv
from iota_sdk import (ClientOptions, CoinType, StrongholdSecretManager, Utils,
                      Wallet, WalletOptions, Bip44, SecretManager)

load_dotenv()

# A name to associate with the created wallet.
ACCOUNT_ALIAS = 'Alice'

# The node to connect to.
node_url = os.environ.get('NODE_URL', 'https://api.testnet.shimmer.network')

# A password to encrypt the stored data.
# WARNING: Never hardcode passwords in production code.
STRONGHOLD_PASSWORD = os.environ.get(
    'STRONGHOLD_PASSWORD', 'a-secure-password')

# The path to store the wallet snapshot.
STRONGHOLD_SNAPSHOT_PATH = 'vault.stronghold'

# Setup Stronghold secret manager
secret_manager = StrongholdSecretManager(
    STRONGHOLD_SNAPSHOT_PATH, STRONGHOLD_PASSWORD)

# Generate a mnemonic and store its seed in the Stronghold vault.
# INFO: It is best practice to back up the mnemonic somewhere secure.
mnemonic = Utils.generate_mnemonic()
print(f'Mnemonic: {mnemonic}')

SecretManager(secret_manager).store_mnemonic(mnemonic)

# Set up and store the wallet.
client_options = ClientOptions(nodes=[node_url])

bib_path = Bip44(
    coin_type=CoinType.SHIMMER
)
wallet_options = WalletOptions(
    None,
    None,
    bib_path,
    client_options,
    secret_manager)

wallet = Wallet(wallet_options)

# Get the first address and print it.
address = wallet.address()
print(f'Address:\n{address}')
