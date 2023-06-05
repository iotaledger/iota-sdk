from iota_sdk import Wallet
from dotenv import load_dotenv
import os
import json

load_dotenv()

# In this example we will send native tokens

wallet = Wallet('./alice-database')

account = wallet.get_account('Alice')

# Sync account with the node
balance = account.sync()

token = [native_balance for native_balance in balance['nativeTokens'] if int(native_balance['available'], 0) >= 10][0]
print(f'Balance before sending: {token["available"]}')

if 'STRONGHOLD_PASSWORD' not in os.environ:
    raise Exception(".env STRONGHOLD_PASSWORD is undefined, see .env.example")

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

outputs = [{
    "address": "rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu",
    "nativeTokens": [(
        token["tokenId"],
        # 10 hex encoded
        "0xA"
    )],
}]

transaction = account.prepare_send_native_tokens(outputs, None).send()
print(f'Transaction sent: {transaction["transactionId"]}')

# Wait for transaction to get included
blockId = account.retry_transaction_until_included(transaction['transactionId'])
print(f'Block included: {os.environ["EXPLORER_URL"]}/block/{blockId}')

balance = account.sync()
available_balance = [native_balance for native_balance in balance['nativeTokens'] if native_balance['tokenId'] == token["tokenId"]][0]['available']
print(f'Balance after minting: {available_balance}')
