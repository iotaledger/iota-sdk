# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

# This example generates an address for an account.

import os

from dotenv import load_dotenv

from iota_sdk import Wallet

# This example uses secrets in environment variables for simplicity which
# should not be done in production.
load_dotenv()

for env_var in ['WALLET_DB_PATH', 'STRONGHOLD_PASSWORD']:
    if env_var not in os.environ:
        raise Exception(f'.env {env_var} is undefined, see .env.example')

wallet = Wallet(os.environ.get('WALLET_DB_PATH'))

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

account = wallet.get_account('Alice')

address = account.generate_ed25519_addresses(1)[0]
print('Address:', address.address)
