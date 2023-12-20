# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

# In this example we create an account and store its addresses in a file which will be used later to find
# inputs.

import json
import os

from dotenv import load_dotenv

from iota_sdk import ClientOptions, CoinType, StrongholdSecretManager, Wallet, WalletOptions, Bip44

load_dotenv()

OFFLINE_WALLET_DB_PATH = "./wallet/offline_signing/example-offline-walletdb"
STRONGHOLD_SNAPSHOT_PATH = "./wallet/offline_signing/example.stronghold"
ADDRESSES_FILE_PATH = "./wallet/offline_signing/example.addresses.json"


node_url = os.environ.get('NODE_URL', 'https://api.testnet.shimmer.network')
offline_client_options = ClientOptions()

for env_var in ['STRONGHOLD_PASSWORD', 'MNEMONIC']:
    if env_var not in os.environ:
        raise Exception(f'.env {env_var} is undefined, see .env.example')

secret_manager = StrongholdSecretManager(
    STRONGHOLD_SNAPSHOT_PATH, os.environ['STRONGHOLD_PASSWORD'])

bib_path = Bip44(
    coin_type=CoinType.SHIMMER
)

wallet_options = WalletOptions(
    None,
    None,
    bib_path,
    offline_client_options,
    secret_manager,
    OFFLINE_WALLET_DB_PATH)
wallet = Wallet(wallet_options)

# Store the mnemonic in the Stronghold snapshot, this only needs to be
# done once
wallet.store_mnemonic(os.environ['MNEMONIC'])

# Get the address from the wallet
address = wallet.address()

json_data = json.dumps(list(map(lambda x: x.__dict__, address)), indent=4)
print(f"example.addresses.json:\n{json_data}")
f = open(ADDRESSES_FILE_PATH, "w", encoding="utf-8")
f.write(json_data)
f.close()
