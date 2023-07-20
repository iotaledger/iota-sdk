# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

# In this example we sign the prepared transaction.

from iota_sdk import Wallet, SecretManager, StrongholdSecretManager, PayloadType, Payload, PreparedTransactionData, SignedTransactionData
from dotenv import load_dotenv
from dacite import from_dict
import json
import os

load_dotenv()

OFFLINE_WALLET_DB_PATH = "./wallet/offline_signing/example-offline-walletdb"
STRONGHOLD_SNAPSHOT_PATH = "./wallet/offline_signing/example.stronghold"
PREPARED_TRANSACTION_FILE_PATH = "./wallet/offline_signing/example.prepared_transaction.json"
SIGNED_TRANSACTION_FILE_PATH = "./wallet/offline_signing/example.signed_transaction.json"


prepared_transaction_data = json.load(
    open(PREPARED_TRANSACTION_FILE_PATH, "r"))
prepared_transaction_data = from_dict(
    PreparedTransactionData, prepared_transaction_data)

wallet = Wallet(OFFLINE_WALLET_DB_PATH)

account = wallet.get_account("Alice")

if 'STRONGHOLD_PASSWORD' not in os.environ:
    raise Exception(".env STRONGHOLD_PASSWORD is undefined, see .env.example")

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

# Signs prepared transaction offline.
signed_transaction_data = account.sign_transaction_essence(
    prepared_transaction_data)

print("Signed transaction.")

json_data = json.dumps(signed_transaction_data.as_dict(), indent=4)
print(f"example.signed_transaction.json:\n{json_data}")
f = open(SIGNED_TRANSACTION_FILE_PATH, "w")
f.write(json_data)
f.close()
