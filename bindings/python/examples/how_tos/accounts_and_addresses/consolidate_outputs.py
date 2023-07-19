from iota_sdk import Wallet, Utils
from dotenv import load_dotenv
import os

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
# same `AddressUnlockCondition`(the first address of the account), so they will be consolidated into one
# output.
outputs = account.unspent_outputs()
print('Outputs BEFORE consolidation:')

for i, output_data in enumerate(outputs):
    print(f'OUTPUT #{i}')
    print(
        '- address: {}\n- amount: {}\n- native tokens: {}'.format(
            Utils.hex_to_bech32(output_data.address.pubKeyHash, 'rms'),
            output_data.output.amount,
            output_data.output.nativeTokens
        )
    )

print('Sending consolidation transaction...')

# Consolidate unspent outputs and print the consolidation transaction ID
# Set `force` to true to force the consolidation even though the
# `output_consolidation_threshold` isn't reached.
transaction = account.prepare_consolidate_outputs(True, None).send()
print('Transaction sent: ', transaction.transactionId)

# Wait for the consolidation transaction to get confirmed
block_id = account.retry_transaction_until_included(transaction.transactionId)

print(
    'Transaction included: {}/block/{}'.format(
        os.environ['EXPLORER_URL'],
        block_id
    )
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
            Utils.hex_to_bech32(output_data.address.pubKeyHash, 'rms'),
            output_data.output.amount,
            output_data.output.nativeTokens
        )
    )
