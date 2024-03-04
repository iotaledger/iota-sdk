import os

from dotenv import load_dotenv
from iota_sdk import SendNativeTokenParams, Wallet, WalletOptions

load_dotenv()

# In this example we will send native tokens

wallet = Wallet(WalletOptions(storage_path=os.environ.get('WALLET_DB_PATH')))

# Sync wallet with the node
balance = wallet.sync()

token_id, token = next(
    (k, v) for k, v in balance.native_tokens.items() if v.available >= 10)
print(f'Balance before sending: {token.available}')

if 'STRONGHOLD_PASSWORD' not in os.environ:
    raise Exception(".env STRONGHOLD_PASSWORD is undefined, see .env.example")

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

outputs = [SendNativeTokenParams(
    "rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu",
    (
        token_id,
        hex(10)
    ),
)]

transaction = wallet.send_native_tokens(outputs, None)
print(f'Transaction sent: {transaction.transaction_id}')

wallet.wait_for_transaction_acceptance(
    transaction.transaction_id)
print(
    f'Tx accepted: {os.environ["EXPLORER_URL"]}/transactions/{transaction.transaction_id}')

balance = wallet.sync()
available_balance = balance.native_tokens[token_id].available
print(f'Balance after sending: {available_balance}')
