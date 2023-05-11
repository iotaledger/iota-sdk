from bindings.python.iota_sdk.types.burn import Burn
from bindings.python.iota_sdk.types.native_token import NativeToken
from iota_sdk import Wallet

# In this example we will burn native tokens

# Explorer url
EXPLORER = "https://explorer.shimmer.network/testnet"

wallet = Wallet("./alice-database")

account = wallet.get_account("Alice")

# Sync account with the node
response = account.sync()
print(f"Synced: {response}")

wallet.set_stronghold_password("some_hopefully_secure_password")

# TODO: replace with your own values.
token_id = (
    "0x08429fe5864378ce70699fc2d22bb144cb86a3c4833d136e3b95c5dadfd6ba0cef0300000000"
)
burn_amount = "0x5"

# Create burn object.
to_burn = Burn().add_native_token(NativeToken(token_id, burn_amount))

# Send transaction.
transaction = account.burn(to_burn)

print(f'Transaction: {transaction["transactionId"]}')
print(f'Block sent: {EXPLORER}/block/" + {transaction["blockId"]}')
