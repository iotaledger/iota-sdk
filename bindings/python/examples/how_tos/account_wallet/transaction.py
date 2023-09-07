from iota_sdk import Wallet, Utils, NodeIndexerAPI, SyncOptions, AccountSyncOptions, SendParams
from dotenv import load_dotenv
import os

# In this example we send funds from an account wallet.

load_dotenv()

sync_options = SyncOptions(alias=AccountSyncOptions(basic_outputs=True))

wallet = Wallet(os.environ['WALLET_DB_PATH'])

account = wallet.get_account('Alice')

if 'STRONGHOLD_PASSWORD' not in os.environ:
    raise Exception(".env STRONGHOLD_PASSWORD is undefined, see .env.example")

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

balance = account.sync(sync_options)

total_base_token_balance = balance.base_coin.total
print(f'Balance before sending funds from account: {total_base_token_balance}')

account_id = balance.accounts[0]
print(f'Account Id: {account_id}')

# Get account address
account_address = Utils.account_id_to_bech32(
    account_id, wallet.get_client().get_bech32_hrp())

# Find first output unlockable by the account address
query_parameters = NodeIndexerAPI.QueryParameters(account_address)
input = wallet.get_client().basic_output_ids(query_parameters).items[0]

params = [SendParams(
    address='rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu',
    amount=1000000,
)]
options = {
    'mandatoryInputs': [input],
}
transaction = account.send_with_params(params, options)
account.reissue_transaction_until_included(
    transaction.transaction_id)
print(
    f'Transaction with custom input: https://explorer.shimmer.network/testnet/transaction/{transaction.transaction_id}')

total_base_token_balance = account.sync(sync_options).base_coin.total
print(f'Balance after sending funds from account: {total_base_token_balance}')
