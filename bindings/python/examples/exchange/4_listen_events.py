# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

# This example listens to the NewOutput event.

from iota_sdk import Wallet, SyncOptions, WalletEventType
from dotenv import load_dotenv
import json
import os
import time

# This example uses secrets in environment variables for simplicity which
# should not be done in production.
load_dotenv()

if 'WALLET_DB_PATH' not in os.environ:
    raise Exception(".env WALLET_DB_PATH is undefined, see .env.example")

wallet = Wallet(os.environ.get('WALLET_DB_PATH'))

account = wallet.get_account('Alice')

received_event = False


def callback(event):
    event_dict = json.loads(event)
    print('AccountIndex:', event_dict["accountIndex"])
    print('Event:', event_dict["event"])

    # Exit after receiving an event.
    global received_event
    received_event = True


# Only interested in new outputs here.
wallet.listen(callback, [WalletEventType.NewOutput])

account = wallet.get_account('Alice')

# Use the faucet to send testnet tokens to your address.
print('Fill your address with the faucet: https://faucet.testnet.shimmer.network/')

addresses = account.addresses()
print('Send funds to:', addresses[0].address)

# Sync every 5 seconds until the faucet transaction gets confirmed.
for _ in range(100):
    if received_event:
        exit()
    time.sleep(5)

    # Sync to detect new outputs
    # Set sync_only_most_basic_outputs to True if not interested in outputs that are timelocked,
    # have a storage deposit return , expiration or are nft/alias/foundry
    # outputs.
    account.sync(SyncOptions(sync_only_most_basic_outputs=True))
