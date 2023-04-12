# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

from iota_wallet import IotaWallet, StrongholdSecretManager

# A name to associate with the created account.
ACCOUNT_ALIAS = 'Alice'

# The node to connect to.
NODE_URL = 'https://api.testnet.shimmer.network'

# The cointype to use.
SHIMMER_COIN_TYPE = 4219

# A password to encrypt the stored data.
# WARNING: Never hardcode passwords in production code.
STRONGHOLD_PASSWORD = 'a-secure-password'

# The path to store the account snapshot.
STRONGHOLD_SNAPSHOT_PATH = 'vault.stronghold'

# Setup Stronghold secret manager
secret_manager = StrongholdSecretManager(STRONGHOLD_SNAPSHOT_PATH, STRONGHOLD_PASSWORD)

# Set up and store the wallet.
client_options = {
    'nodes': [NODE_URL],
}

wallet = IotaWallet(
    client_options=client_options,
    coin_type=SHIMMER_COIN_TYPE,
    secret_manager=secret_manager
)

# Generate a mnemonic and store it in the Stronghold vault.
# INFO: It is best practice to back up the mnemonic somewhere secure.
mnemonic = wallet.generate_mnemonic()
wallet.store_mnemonic(mnemonic)

# Create an account.
wallet.create_account('Alice')

# Get the first address and print it.
address = wallet.get_account('Alice').addresses()[0]
print(f'Address:\n{address["address"]}\n')
