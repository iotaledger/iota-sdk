from iota_sdk import Wallet, Utils, SyncOptions, AliasSyncOptions
from dotenv import load_dotenv
import os
import time

# In this example we request funds to an alias wallet.

load_dotenv()

FAUCET_URL = os.environ.get(
    'FAUCET_URL', 'https://faucet.testnet.shimmer.network/api/enqueue')

wallet = Wallet(os.environ['WALLET_DB_PATH'])

account = wallet.get_account('Alice')
balance = account.sync(None)

total_base_token_balance = balance.baseCoin.total
print(
    f'Balance before requesting funds on alias address: {total_base_token_balance}')

alias_id = balance.aliases[0]
print(f'Alias Id: {alias_id}')

# Get Alias address
alias_address = Utils.alias_id_to_bech32(
    alias_id, wallet.get_client().get_bech32_hrp())
faucet_response = wallet.get_client().request_funds_from_faucet(
    FAUCET_URL, alias_address)
print(faucet_response)

time.sleep(10)

sync_options = SyncOptions(alias=AliasSyncOptions(basic_outputs=True))

total_base_token_balance = account.sync(sync_options).baseCoin.total
print(
    f'Balance after requesting funds on alias address: {total_base_token_balance}')
