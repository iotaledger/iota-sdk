from iota_sdk import Wallet, TransactionOptions, TaggedDataPayload, utf8_to_hex, RemainderValueStrategy
from dotenv import load_dotenv
import os

load_dotenv()

# This example sends a transaction with a tagged data payload.

wallet = Wallet('./alice-database')

account = wallet.get_account('Alice')

# Sync account with the node
response = account.sync()

if 'STRONGHOLD_PASSWORD' not in os.environ:
    raise Exception(".env STRONGHOLD_PASSWORD is undefined, see .env.example")

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

outputs = [{
    "address": "rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu",
    "amount": "1000000",
}]

transaction = account.send_amount(outputs, TransactionOptions(remainder_value_strategy=RemainderValueStrategy.ReuseAddress,
                                  note="my first tx", tagged_data_payload=TaggedDataPayload(utf8_to_hex("tag"), utf8_to_hex("data"))))
print(transaction)
print(f'Block sent: {os.environ["EXPLORER_URL"]}/block/{transaction.blockId}')
