import os

from dotenv import load_dotenv

from iota_sdk import ConsolidationParams, Utils, Wallet

# In this example we will consolidate basic outputs from an account with only an AddressUnlockCondition by sending
# them to the same address again.

# This example uses secrets in environment variables for simplicity which
# should not be done in production.
load_dotenv()

if 'STRONGHOLD_PASSWORD' not in os.environ:
    raise Exception('.env STRONGHOLD_PASSWORD is undefined, see .env.example')

wallet = Wallet(os.environ['WALLET_DB_PATH'])
wallet.set_stronghold_password(os.environ['STRONGHOLD_PASSWORD'])

account = wallet.get_account('Alice')

# Sync account to make sure account is updated with outputs from previous
# examples.
account.sync()
print('Account synced')

# List unspent outputs before consolidation.
# The output we created with example `request_funds` and the basic output from `mint` have only one
# unlock condition and it is an `AddressUnlockCondition`, and so they are valid for consolidation. They have the
# same `AddressUnlockCondition`(the address of the wallet), so they will be consolidated into one
# output.
outputs = account.unspent_outputs()
print('Outputs BEFORE consolidation:')

for i, output_data in enumerate(outputs):
    print(f'OUTPUT #{i}')
    print(
        '- address: {}\n- amount: {}\n- native tokens: {}'.format(
            Utils.hex_to_bech32(output_data.address.pub_key_hash, 'rms'),
            output_data.output.amount,
            output_data.output.native_tokens
        )
    )

print('Sending consolidation transaction...')

# Consolidate unspent outputs and print the consolidation transaction ID
# Set `force` to true to force the consolidation even though the
# `output_threshold` isn't reached.
transaction = account.consolidate_outputs(ConsolidationParams(force=True))
print('Transaction sent: ', transaction.transaction_id)

# Wait for the consolidation transaction to get confirmed
block_id = account.reissue_transaction_until_included(
    transaction.transaction_id)

print(
    f'Transaction included: {os.environ["EXPLORER_ID"]}/block/{block_id}'
)

# Sync account
account.sync()
print('Account synced')

# Outputs after consolidation
outputs = account.unspent_outputs()
print('Outputs AFTER consolidation:')
for i, output_data in enumerate(outputs):
    print(f'OUTPUT #{i}')
    print(
        '- address: {}\n- amount: {}\n- native tokens: {}'.format(
            Utils.hex_to_bech32(output_data.address.pub_key_hash, 'rms'),
            output_data.output.amount,
            output_data.output.native_tokens
        )
    )
