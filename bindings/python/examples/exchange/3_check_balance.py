# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

# This example gets the balance of an account.

from iota_sdk import Wallet, SyncOptions
from dotenv import load_dotenv
import os

# This example uses secrets in environment variables for simplicity which
# should not be done in production.
load_dotenv()

if 'WALLET_DB_PATH' not in os.environ:
    raise Exception(".env WALLET_DB_PATH is undefined, see .env.example")

wallet = Wallet(os.environ.get('WALLET_DB_PATH'))

account = wallet.get_account('Alice')

addresses = account.addresses()
print(f'Addresses:', addresses)

# Set sync_only_most_basic_outputs to True if not interested in outputs that are timelocked,
# have a storage deposit return, expiration or are nft/alias/foundry outputs.
balance = account.sync(SyncOptions(sync_only_most_basic_outputs=True))
print('Balance', balance)

# Use the faucet to send tokens to your address.
print('Fill your address with the Faucet: https://faucet.testnet.shimmer.network/')
