# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

# This example listens to the NewOutput event.

import json
import os
import sys
import time

from dotenv import load_dotenv
from iota_sdk import SyncOptions, Wallet, WalletOptions, WalletEventType

# This example uses secrets in environment variables for simplicity which
# should not be done in production.
load_dotenv()

for env_var in ['WALLET_DB_PATH', 'FAUCET_URL']:
    if env_var not in os.environ:
        raise Exception(".env WALLET_DB_PATH is undefined, see .env.example")

wallet = Wallet(WalletOptions(storage_path=os.environ.get('WALLET_DB_PATH')))

received_event = False


def callback(event):
    """Callback function for the event listener"""
    event = json.loads(event)
    print('Event:', event)

    # Exit after receiving an event.
    # pylint: disable=global-statement
    global received_event
    received_event = True


# Only interested in new outputs here.
wallet.listen(callback, [WalletEventType.NewOutput])

# Use the faucet to send testnet tokens to your address.
print(f'Fill your address with the Faucet: {os.environ["FAUCET_URL"]}')

address = wallet.address()
print('Send funds to:', address)

# Sync every 5 seconds until the faucet transaction gets confirmed.
for _ in range(100):
    if received_event:
        sys.exit()
    time.sleep(5)

    # Sync to detect new outputs
    # Set sync_only_most_basic_outputs to True if not interested in outputs that are timelocked,
    # have a storage deposit return , expiration or are nft/account/foundry
    # outputs.
    wallet.sync(SyncOptions(sync_only_most_basic_outputs=True))
