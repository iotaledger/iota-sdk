import os

from dotenv import load_dotenv

from iota_sdk import SendNativeTokenParams, Wallet, WalletOptions

load_dotenv()

# In this example we will send native tokens

wallet = Wallet(WalletOptions(storage_path=os.environ.get('WALLET_DB_PATH')))

# Sync wallet with the node
balance = wallet.sync()

token = [native_balance for native_balance in balance.native_tokens if int(
    native_balance.available, 0) >= 10][0]
print(f'Balance before sending: {int(token.available, 0)}')

if 'STRONGHOLD_PASSWORD' not in os.environ:
    raise Exception(".env STRONGHOLD_PASSWORD is undefined, see .env.example")

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

outputs = [SendNativeTokenParams(
    "rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu",
    (
        token.token_id,
        hex(10)
    ),
)]

transaction = wallet.send_native_tokens(outputs, None)
print(f'Transaction sent: {transaction.transaction_id}')

# Wait for transaction to get included
block_id = wallet.reissue_transaction_until_included(
    transaction.transaction_id)
print(f'Block included: {os.environ["EXPLORER_URL"]}/block/{block_id}')

balance = wallet.sync()
available_balance = int(
    [native_balance for native_balance in balance.native_tokens if native_balance.token_id == token.token_id][0].available, 0)
print(f'Balance after sending: {available_balance}')
