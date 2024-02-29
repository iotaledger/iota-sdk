import os

from dotenv import load_dotenv
from iota_sdk import (RemainderValueStrategy, TaggedDataPayload, SendParams,
                      TransactionOptions, Wallet, WalletOptions, utf8_to_hex)

load_dotenv()

# This example sends a transaction with a tagged data payload.

wallet = Wallet(WalletOptions(storage_path=os.environ.get('WALLET_DB_PATH')))

# Sync account with the node
response = wallet.sync()

if 'STRONGHOLD_PASSWORD' not in os.environ:
    raise Exception(".env STRONGHOLD_PASSWORD is undefined, see .env.example")

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

params = [SendParams(
    address="rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu",
    amount=1000000,
)]

transaction = wallet.send_with_params(
    params,
    TransactionOptions(
        remainder_value_strategy=RemainderValueStrategy.ReuseAddress,
        note="my first tx",
        tagged_data_payload=TaggedDataPayload(
            utf8_to_hex("tag"),
             utf8_to_hex("data"))))
print(transaction)
print(f'Block sent: {os.environ["EXPLORER_URL"]}/block/{transaction.block_id}')
