# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

# This example gets the balance of a wallet.

import os

from dotenv import load_dotenv
from iota_sdk import SyncOptions, Wallet, WalletOptions

# This example uses secrets in environment variables for simplicity which
# should not be done in production.
load_dotenv()

for env_var in ['WALLET_DB_PATH', 'FAUCET_URL']:
    if env_var not in os.environ:
        raise Exception(f'.env {env_var} is undefined, see .env.example')

wallet = Wallet(WalletOptions(storage_path=os.environ.get('WALLET_DB_PATH')))

address = wallet.address()
print('Address:', address)

# Set sync_only_most_basic_outputs to True if not interested in outputs that are timelocked,
# have a storage deposit return, expiration or are nft/account/foundry outputs.
balance = wallet.sync(SyncOptions(sync_only_most_basic_outputs=True))
print('Balance', balance)

# Use the faucet to send tokens to your address.
print(f'Fill your address with the Faucet: {os.environ["FAUCET_URL"]}')
