from iota_sdk import Wallet, HexStr
from dotenv import load_dotenv
import os

load_dotenv()

# In this example we will burn native tokens

wallet = Wallet(os.environ['WALLET_DB_PATH'])

account = wallet.get_account('Alice')

# Sync account with the node
balance = account.sync()

if 'STRONGHOLD_PASSWORD' not in os.environ:
    raise Exception(".env STRONGHOLD_PASSWORD is undefined, see .env.example")

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

# Find native token with enough balance
token = [
    native_balance for native_balance in balance.nativeTokens if int(
        native_balance.available,
        0) >= 10][0]
print(f'Balance before burning: {int(token.available, 0)}')

burn_amount = 1

# Send transaction.
transaction = account.prepare_burn_native_token(
    token.tokenId, burn_amount).send()
print(f'Transaction sent: {transaction.transactionId}')

# Wait for transaction to get included
blockId = account.retry_transaction_until_included(transaction.transactionId)
print(f'Block included: {os.environ["EXPLORER_URL"]}/block/{blockId}')

balance = account.sync()
available_balance = int(
    [native_balance for native_balance in balance.nativeTokens if native_balance.tokenId == token.tokenId][0].available, 0)
print(f'Balance after burning: {available_balance}')
