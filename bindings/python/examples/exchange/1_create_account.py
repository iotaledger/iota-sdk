# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

# This example creates a new database and account.

import os

from dotenv import load_dotenv

from iota_sdk import (ClientOptions, CoinType, StrongholdSecretManager,
                      SyncOptions, Wallet)

# This example uses secrets in environment variables for simplicity which
# should not be done in production.
load_dotenv()

for env_var in ['WALLET_DB_PATH', 'NODE_URL', 'STRONGHOLD_SNAPSHOT_PATH', 'STRONGHOLD_PASSWORD', 'MNEMONIC']:
    if env_var not in os.environ:
        raise Exception(f'.env {env_var} is undefined, see .env.example')

client_options = ClientOptions(nodes=[os.environ.get('NODE_URL')])

secret_manager = StrongholdSecretManager(
    os.environ.get('STRONGHOLD_SNAPSHOT_PATH'), os.environ['STRONGHOLD_PASSWORD'])

wallet = Wallet(os.environ.get('WALLET_DB_PATH'),
                client_options, CoinType.IOTA, secret_manager)

# Store the mnemonic in the Stronghold snapshot, this only needs to be
# done once.
wallet.store_mnemonic(
    os.environ['MNEMONIC'])

account = wallet.create_account('Alice')

# Set sync_only_most_basic_outputs to True if not interested in outputs that are timelocked,
# have a storage deposit return, expiration or are nft/alias/foundry outputs.
account.set_default_sync_options(
    SyncOptions(sync_only_most_basic_outputs=True))

print(account.get_metadata())
