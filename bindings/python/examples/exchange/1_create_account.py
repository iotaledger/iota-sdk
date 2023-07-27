# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

# This example creates a new database and account.

from iota_sdk import Wallet, StrongholdSecretManager, SyncOptions, CoinType, ClientOptions
from dotenv import load_dotenv
import os

# This example uses secrets in environment variables for simplicity which
# should not be done in production.
load_dotenv()

if 'WALLET_DB_PATH' not in os.environ:
    raise Exception(".env WALLET_DB_PATH is undefined, see .env.example")
if 'NODE_URL' not in os.environ:
    raise Exception(".env NODE_URL is undefined, see .env.example")
if 'STRONGHOLD_SNAPSHOT_PATH' not in os.environ:
    raise Exception(
        ".env STRONGHOLD_SNAPSHOT_PATH is undefined, see .env.example")
if 'STRONGHOLD_PASSWORD' not in os.environ:
    raise Exception(".env STRONGHOLD_PASSWORD is undefined, see .env.example")
if 'MNEMONIC' not in os.environ:
    raise Exception(".env MNEMONIC is undefined, see .env.example")

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
