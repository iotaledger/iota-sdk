from iota_sdk import Wallet, HexStr
from dotenv import load_dotenv
import os

load_dotenv()

# In this example we will decrease the native token supply

wallet = Wallet(os.environ['WALLET_DB_PATH'])

account = wallet.get_account('Alice')

# Sync account with the node
balance = account.sync()

# Find first foundry and corresponding token id
token_id = balance.foundries[0]

available_balance = int(
    [native_balance for native_balance in balance.nativeTokens if native_balance.tokenId == token_id][0].available, 0)
print(f'Balance before melting: {available_balance}')

if 'STRONGHOLD_PASSWORD' not in os.environ:
    raise Exception(".env STRONGHOLD_PASSWORD is undefined, see .env.example")

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

melt_amount = 10

# Send transaction.
transaction = account.prepare_melt_native_token(token_id, melt_amount).send()
print(f'Transaction sent: {transaction.transactionId}')

# Wait for transaction to get included
blockId = account.retry_transaction_until_included(
    transaction['transactionId'])
print(f'Block included: {os.environ["EXPLORER_URL"]}/block/{blockId}')

balance = account.sync()
available_balance = int(
    [native_balance for native_balance in balance.nativeTokens if native_balance.tokenId == token_id][0].available, 0)
print(f'Balance after melting: {available_balance}')
