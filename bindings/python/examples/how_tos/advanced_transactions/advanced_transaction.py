import datetime
import os
import time

from dotenv import load_dotenv
from iota_sdk import (
    AddressUnlockCondition,
    Client,
    Ed25519Address,
    Wallet,
    WalletOptions,
    Utils,
    TimelockUnlockCondition,
)

# This example uses secrets in environment variables for simplicity which
# should not be done in production.
load_dotenv()

# This example sends a transaction with a timelock.

for env_var in ['WALLET_DB_PATH', 'STRONGHOLD_PASSWORD']:
    if env_var not in os.environ:
        raise Exception(f'.env {env_var} is undefined, see .env.example')

wallet = Wallet(WalletOptions(storage_path=os.environ.get('WALLET_DB_PATH')))

# Sync wallet with the node
wallet.sync()

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

# Create an output with amount 1_000_000 and a timelock of 1 hour
in_an_hour = int(
    time.mktime(
        (datetime.datetime.now() +
         datetime.timedelta(
            hours=1)).timetuple()))
basic_output = Client().build_basic_output(
    unlock_conditions=[
        AddressUnlockCondition(
            Ed25519Address(
                Utils.bech32_to_hex('rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy'))
        ),
        TimelockUnlockCondition(in_an_hour),
    ],
)

transaction = wallet.send_outputs([basic_output])
print(f'Transaction sent: {transaction.transaction_id}')

wallet.wait_for_transaction_acceptance(
    transaction.transaction_id)

print(
    f'Tx accepted: {os.environ["EXPLORER_URL"]}/transactions/{transaction.transaction_id}')
