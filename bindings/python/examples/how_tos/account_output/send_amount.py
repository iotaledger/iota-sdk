import os

from dotenv import load_dotenv
from iota_sdk import AccountAddress, Wallet, WalletOptions, Utils, NodeIndexerAPI, SyncOptions, WalletSyncOptions, SendParams

# In this example we send funds from an account output.

load_dotenv()

for env_var in ['WALLET_DB_PATH', 'STRONGHOLD_PASSWORD']:
    if env_var not in os.environ:
        raise Exception(f".env {env_var} is undefined, see .env.example")

wallet = Wallet(WalletOptions(storage_path=os.environ.get('WALLET_DB_PATH')))

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

sync_options = SyncOptions(wallet=WalletSyncOptions(basic_outputs=True))
balance = wallet.sync(sync_options)

total_base_token_balance = balance.base_coin.total
print(
    f'Balance before sending funds from the account output: {total_base_token_balance}')

account_id = balance.accounts[0]
print(f'Account Id: {account_id}')

# Get account address
account_address = Utils.address_to_bech32(
    AccountAddress(account_id), wallet.get_client().get_bech32_hrp())

# Find first output unlockable by the account address
query_parameters = NodeIndexerAPI.BasicOutputQueryParameters(
    address=account_address)
inputs = [wallet.get_client().basic_output_ids(query_parameters).items[0]]

params = [SendParams(
    address='rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu',
    amount=1000000,
)]
options = {
    'requiredInputs': inputs,
}
transaction = wallet.send_with_params(params, options)
wallet.wait_for_transaction_acceptance(
    transaction.transaction_id)
print(
    f'Transaction with custom input: https://explorer.shimmer.network/testnet/transaction/{transaction.transaction_id}')

total_base_token_balance = wallet.sync(sync_options).base_coin.total
print(
    f'Balance after sending funds from the account output: {total_base_token_balance}')
