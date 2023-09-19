# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

# In this example we will get inputs and prepare a transaction.

import json
import os

from dacite import from_dict
from dotenv import load_dotenv

from iota_sdk import (AccountAddress, ClientOptions, CoinType, SendParams,
                      Wallet)

load_dotenv()

ONLINE_WALLET_DB_PATH = "./wallet/offline_signing/example-online-walletdb"
ADDRESSES_FILE_PATH = "./wallet/offline_signing/example.addresses.json"
PREPARED_TRANSACTION_FILE_PATH = "./wallet/offline_signing/example.prepared_transaction.json"
# Address to which we want to send the amount.
RECV_ADDRESS = "rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu"
# The amount to send.
SEND_AMOUNT = 1_000_000


params = [SendParams(RECV_ADDRESS, str(SEND_AMOUNT))]

# Recovers addresses from example `0_address_generation`.
addresses_data = json.load(open(ADDRESSES_FILE_PATH, "r", encoding="utf-8"))
addresses = list(map(lambda x: from_dict(
    data_class=AccountAddress, data=x), addresses_data))

if 'NODE_URL' not in os.environ:
    raise Exception(".env NODE_URL is undefined, see .env.example")

client_options = ClientOptions(nodes=[os.environ.get('NODE_URL')])

wallet = Wallet(ONLINE_WALLET_DB_PATH, client_options,
                CoinType.IOTA, "placeholder")

account = wallet.create_account('Alice', "rms", addresses)
print("Account created:", account.get_metadata())

account.sync()

prepared_transaction = account.prepare_send(params)

json_data = json.dumps(
    prepared_transaction.prepared_transaction_data().to_dict(),
    indent=4)
print(f"example.prepared_transaction.json:\n{json_data}")
f = open(PREPARED_TRANSACTION_FILE_PATH, "w", encoding="utf-8")
f.write(json_data)
f.close()
