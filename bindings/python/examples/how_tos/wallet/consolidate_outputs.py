import os

from dotenv import load_dotenv
from iota_sdk import ConsolidationParams, Utils, Wallet, WalletOptions, FeatureType

# In this example we will consolidate basic outputs from a wallet with only an AddressUnlockCondition by sending
# them to the same address again.

# This example uses secrets in environment variables for simplicity which
# should not be done in production.
load_dotenv()

if 'STRONGHOLD_PASSWORD' not in os.environ:
    raise Exception('.env STRONGHOLD_PASSWORD is undefined, see .env.example')

wallet = Wallet(WalletOptions(storage_path=os.environ.get('WALLET_DB_PATH')))
wallet.set_stronghold_password(os.environ['STRONGHOLD_PASSWORD'])

# Sync wallet to make sure it is updated with outputs from the previous
# examples.
wallet.sync()
print('Wallet synced')

# List unspent outputs before consolidation.
# The output we created with example `request_funds` and the basic output from `mint` have only one
# unlock condition and it is an `AddressUnlockCondition`, and so they are valid for consolidation. They have the
# same `AddressUnlockCondition`(the address of the wallet), so they will be consolidated into one
# output.
outputs = wallet.unspent_outputs()
print('Outputs BEFORE consolidation:')

for i, output_data in enumerate(outputs):
    print(f'OUTPUT #{i}')
    print(
        f'- address: #{Utils.hex_to_bech32(output_data.address.pub_key_hash, "rms")}')
    print(f'- amount: #{output_data.output.amount}')

    native_tokens = [
        feature for feature in output_data.output.features if feature.type == FeatureType.NativeToken]
    opt_native_token = next(iter(native_tokens), None)
    print(f'- native tokens: #{opt_native_token}')

print('Sending consolidation transaction...')

# Consolidate unspent outputs and print the consolidation transaction ID
# Set `force` to true to force the consolidation even though the
# `output_threshold` isn't reached.
transaction = wallet.consolidate_outputs(ConsolidationParams(force=True))
print('Transaction sent: ', transaction.transaction_id)

# Wait for the consolidation transaction to get accepted
wallet.wait_for_transaction_acceptance(
    transaction.transaction_id)

print(
    f'Tx accepted: {os.environ["EXPLORER_URL"]}/transactions/{transaction.transaction_id}')

# Sync wallet
wallet.sync()
print('Wallet synced')

# Outputs after consolidation
outputs = wallet.unspent_outputs()
print('Outputs AFTER consolidation:')
for i, output_data in enumerate(outputs):
    print(f'OUTPUT #{i}')
    print(
        f'- address: #{Utils.hex_to_bech32(output_data.address.pub_key_hash, "rms")}')
    print(f'- amount: #{output_data.output.amount}')

    native_tokens = [
        feature for feature in output_data.output.features if feature.type == FeatureType.NativeToken]
    opt_native_token = next(iter(native_tokens), None)
    print(f'- native tokens: #{opt_native_token}')
