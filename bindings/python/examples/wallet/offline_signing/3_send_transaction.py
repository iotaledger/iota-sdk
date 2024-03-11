# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

# In this example we send the signed transaction in a block.

import json
import os

from dacite import from_dict
from dotenv import load_dotenv
from iota_sdk import SignedTransactionData, Wallet, WalletOptions

load_dotenv()

ONLINE_WALLET_DB_PATH = "./wallet/offline_signing/example-online-walletdb"
SIGNED_TRANSACTION_FILE_PATH = "./wallet/offline_signing/example.signed_transaction.json"

if 'EXPLORER_URL' not in os.environ:
    raise Exception(".env EXPLORER_URL is undefined, see .env.example")

wallet = Wallet(WalletOptions(storage_path=ONLINE_WALLET_DB_PATH))

signed_transaction_data = json.load(
    open(SIGNED_TRANSACTION_FILE_PATH, "r", encoding="utf-8"))
signed_transaction_data = from_dict(
    SignedTransactionData, signed_transaction_data)

# Sends offline signed transaction online.
transaction = wallet.submit_and_store_transaction(signed_transaction_data)
print(
    f'Transaction sent: {os.environ["EXPLORER_URL"]}/transactions/{transaction.transaction_id}')
wallet.wait_for_transaction_acceptance(
    transaction.transaction_id)
print(
    f'Tx accepted: {os.environ["EXPLORER_URL"]}/transactions/{transaction.transaction_id}')
