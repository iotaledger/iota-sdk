# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

# In this example we generate addresses which will be used later to find
# inputs.

from iota_sdk import Wallet, StrongholdSecretManager, CoinType, ClientOptions
from dotenv import load_dotenv
import json
import os

load_dotenv()

OFFLINE_WALLET_DB_PATH = "./wallet/offline_signing/example-offline-walletdb"
STRONGHOLD_SNAPSHOT_PATH = "./wallet/offline_signing/example.stronghold"
ADDRESSES_FILE_PATH = "./wallet/offline_signing/example.addresses.json"


node_url = os.environ.get('NODE_URL', 'https://api.testnet.shimmer.network')
offline_client_options = ClientOptions()

if 'STRONGHOLD_PASSWORD' not in os.environ:
    raise Exception(".env STRONGHOLD_PASSWORD is undefined, see .env.example")

secret_manager = StrongholdSecretManager(
    STRONGHOLD_SNAPSHOT_PATH, os.environ['STRONGHOLD_PASSWORD'])

wallet = Wallet(OFFLINE_WALLET_DB_PATH, offline_client_options,
                CoinType.IOTA, secret_manager)

if 'MNEMONIC' not in os.environ:
    raise Exception(".env MNEMONIC is undefined, see .env.example")

# Store the mnemonic in the Stronghold snapshot, this only needs to be
# done once
wallet.store_mnemonic(os.environ['MNEMONIC'])

account = wallet.create_account('Alice', "rms")
print("Account created:", account.get_metadata())

addresses = account.addresses()

json_data = json.dumps(list(map(lambda x: x.__dict__, addresses)), indent=4)
print(f"example.addresses.json:\n{json_data}")
f = open(ADDRESSES_FILE_PATH, "w")
f.write(json_data)
f.close()
