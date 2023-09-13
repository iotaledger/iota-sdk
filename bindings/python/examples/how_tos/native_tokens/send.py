import os

from dotenv import load_dotenv

from iota_sdk import SendNativeTokensParams, Wallet

load_dotenv()

# In this example we will send native tokens

wallet = Wallet(os.environ['WALLET_DB_PATH'])

account = wallet.get_account('Alice')

# Sync account with the node
balance = account.sync()

token = [native_balance for native_balance in balance.native_tokens if int(
    native_balance.available, 0) >= 10][0]
print(f'Balance before sending: {int(token.available, 0)}')

if 'STRONGHOLD_PASSWORD' not in os.environ:
    raise Exception(".env STRONGHOLD_PASSWORD is undefined, see .env.example")

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

outputs = [SendNativeTokensParams(
    "rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu",
    [(
        token.token_id,
        hex(10)
    )],
)]

transaction = account.send_native_tokens(outputs, None)
print(f'Transaction sent: {transaction.transaction_id}')

# Wait for transaction to get included
block_id = account.reissue_transaction_until_included(
    transaction.transaction_id)
print(f'Block included: {os.environ["EXPLORER_URL"]}/block/{block_id}')

balance = account.sync()
available_balance = int(
    [native_balance for native_balance in balance.native_tokens if native_balance.token_id == token.token_id][0].available, 0)
print(f'Balance after sending: {available_balance}')
