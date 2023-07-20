from iota_sdk import *
from dotenv import load_dotenv
import os
import time
import datetime

load_dotenv()

# This example sends a transaction with a timelock.

wallet = Wallet(os.environ['WALLET_DB_PATH'])

account = wallet.get_account('Alice')

# Sync account with the node
response = account.sync()

if 'STRONGHOLD_PASSWORD' not in os.environ:
    raise Exception(".env STRONGHOLD_PASSWORD is undefined, see .env.example")

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

# Create an ouput with amount 1_000_000 and a timelock of 1 hour
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

transaction = account.send_outputs([basic_output])
print(f'Transaction sent: {transaction.transactionId}')

block_id = account.retry_transaction_until_included(transaction.transactionId)

print(
    f'Block sent: {os.environ["EXPLORER_URL"]}/block/{block_id}')
