# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

# This example generates an address for a wallet.

import os

from dotenv import load_dotenv
from iota_sdk import StrongholdSecretManager, SecretManager

# This example uses secrets in environment variables for simplicity which
# should not be done in production.
load_dotenv()

for env_var in ['STRONGHOLD_SNAPSHOT_PATH', 'STRONGHOLD_PASSWORD']:
    if env_var not in os.environ:
        raise Exception(f'.env {env_var} is undefined, see .env.example')

secret_manager = SecretManager(StrongholdSecretManager(
    os.environ.get('STRONGHOLD_SNAPSHOT_PATH'), os.environ['STRONGHOLD_PASSWORD']))

address = secret_manager.generate_ed25519_addresses(1)[0]
print('Address:', address)
