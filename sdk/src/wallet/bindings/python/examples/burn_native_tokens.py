from iota_wallet import IotaWallet

# In this example we will burn native tokens

# Explorer url
EXPLORER = "https://explorer.shimmer.network/testnet"

wallet = IotaWallet('./alice-database')

account = wallet.get_account('Alice')

# Sync account with the node
response = account.sync()
print(f'Synced: {response}')

wallet.set_stronghold_password("some_hopefully_secure_password")

# TODO: replace with your own values.
token_id = "0x08429fe5864378ce70699fc2d22bb144cb86a3c4833d136e3b95c5dadfd6ba0cef0300000000"
burn_amount = "0x5"

# Send transaction.
transaction = account.burn_native_token(token_id, burn_amount)

print(f'Transaction: {transaction.transaction_id}')
print(f'Block sent: {EXPLORER}/block/" + {transaction.block_id});
