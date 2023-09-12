# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

# In this example we send the signed transaction in a block.

import json
import os

from dacite import from_dict
from dotenv import load_dotenv

from iota_sdk import SignedTransactionData, Wallet

load_dotenv()

ONLINE_WALLET_DB_PATH = "./wallet/offline_signing/example-online-walletdb"
SIGNED_TRANSACTION_FILE_PATH = "./wallet/offline_signing/example.signed_transaction.json"

if 'EXPLORER_URL' not in os.environ:
    raise Exception(".env EXPLORER_URL is undefined, see .env.example")

wallet = Wallet(ONLINE_WALLET_DB_PATH, None, None, "placeholder")

account = wallet.get_account("Alice")

signed_transaction_data = json.load(
    open(SIGNED_TRANSACTION_FILE_PATH, "r", encoding="utf-8"))
signed_transaction_data = from_dict(
    SignedTransactionData, signed_transaction_data)

# Sends offline signed transaction online.
transaction = account.submit_and_store_transaction(signed_transaction_data)
print(
    f'Transaction sent: {os.environ["EXPLORER_URL"]}/transaction/{transaction.transaction_id}')
block_id = account.reissue_transaction_until_included(
    transaction.transaction_id)
print(f'Block included: {os.environ["EXPLORER_URL"]}/block/{block_id}')
