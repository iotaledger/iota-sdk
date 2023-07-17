from iota_sdk import Wallet, Utils, NodeIndexerAPI, SyncOptions, AliasSyncOptions, SendParams
from dotenv import load_dotenv
import os

# In this example we send funds from an alias wallet.

load_dotenv()

sync_options = SyncOptions(alias=AliasSyncOptions(basic_outputs=True))

wallet = Wallet(os.environ['WALLET_DB_PATH'])

account = wallet.get_account('Alice')

if 'STRONGHOLD_PASSWORD' not in os.environ:
    raise Exception(".env STRONGHOLD_PASSWORD is undefined, see .env.example")

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

balance = account.sync(sync_options)

total_base_token_balance = balance.baseCoin.total
print(f'Balance before sending funds from alias: {total_base_token_balance}')

alias_id = balance.aliases[0]
print(f'Alias Id: {alias_id}')

# Get alias address
alias_address = Utils.alias_id_to_bech32(
    alias_id, wallet.get_client().get_bech32_hrp())

# Find first output unlockable by the alias address
query_parameters = NodeIndexerAPI.QueryParameters(alias_address)
input = wallet.get_client().basic_output_ids(query_parameters).items[0]

params = [SendParams(
    address='rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu',
    amount=1000000,
)]
options = {
    'mandatoryInputs': [input],
}
transaction = account.send(params, options)
account.retry_transaction_until_included(transaction.transactionId, None, None)
print(
    f'Transaction with custom input: https://explorer.shimmer.network/testnet/transaction/{transaction.transactionId}')

total_base_token_balance = account.sync(sync_options).baseCoin.total
print(f'Balance after sending funds from alias: {total_base_token_balance}')
