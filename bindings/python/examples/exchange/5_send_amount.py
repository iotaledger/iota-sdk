# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

# This example sends tokens to an address.

from iota_sdk import Wallet, SendParams, SyncOptions
from dotenv import load_dotenv
import os

# This example uses secrets in environment variables for simplicity which should not be done in production.
load_dotenv()

if 'WALLET_DB_PATH' not in os.environ:
    raise Exception(".env WALLET_DB_PATH is undefined, see .env.example")
if 'STRONGHOLD_PASSWORD' not in os.environ:
    raise Exception(".env STRONGHOLD_PASSWORD is undefined, see .env.example")
if 'EXPLORER_URL' not in os.environ:
    raise Exception(".env EXPLORER_URL is undefined, see .env.example")

wallet = Wallet(os.environ.get('WALLET_DB_PATH'))

account = wallet.get_account('Alice')

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

# Set sync_only_most_basic_outputs to True if not interested in outputs that are timelocked,
# have a storage deposit return, expiration or are nft/alias/foundry outputs.
balance = account.sync(SyncOptions(sync_only_most_basic_outputs=True))
print('Balance', balance)

transaction = account.send([SendParams(
    "rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu",
    1000000,
)])
print(transaction)
print(
    f'Check your block on: {os.environ["EXPLORER_URL"]}/block/{transaction.blockId}')
