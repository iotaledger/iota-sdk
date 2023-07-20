# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

# This example generates an address for an account.

from iota_sdk import Wallet
from dotenv import load_dotenv
import os

# This example uses secrets in environment variables for simplicity which
# should not be done in production.
load_dotenv()

if 'WALLET_DB_PATH' not in os.environ:
    raise Exception(".env WALLET_DB_PATH is undefined, see .env.example")
if 'STRONGHOLD_PASSWORD' not in os.environ:
    raise Exception(".env STRONGHOLD_PASSWORD is undefined, see .env.example")

wallet = Wallet(os.environ.get('WALLET_DB_PATH'))

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

account = wallet.get_account('Alice')

address = account.generate_ed25519_addresses(1)[0]
print(f'Address:', address.address)
