import os
import time

from dotenv import load_dotenv

from iota_sdk import Wallet, Utils, SyncOptions, AccountSyncOptions

# In this example we request funds to an account wallet.

load_dotenv()

FAUCET_URL = os.environ.get(
    'FAUCET_URL', 'https://faucet.testnet.shimmer.network/api/enqueue')

wallet = Wallet(os.environ['WALLET_DB_PATH'])

account = wallet.get_account('Alice')
balance = account.sync(None)

total_base_token_balance = balance.base_coin.total
print(
    f'Balance before requesting funds on account address: {total_base_token_balance}')

account_id = balance.accounts[0]
print(f'Account Id: {account_id}')

# Get Account address
account_address = Utils.account_id_to_bech32(
    account_id, wallet.get_client().get_bech32_hrp())
faucet_response = wallet.get_client().request_funds_from_faucet(
    FAUCET_URL, account_address)
print(faucet_response)

time.sleep(10)

sync_options = SyncOptions(alias=AccountSyncOptions(basic_outputs=True))

total_base_token_balance = account.sync(sync_options).base_coin.total
print(
    f'Balance after requesting funds on account address: {total_base_token_balance}')
