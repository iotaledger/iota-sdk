from bindings.python.iota_sdk.types.burn import Burn
from iota_sdk import Wallet

# In this example we will destroy an alias output

# TODO: replace with your own values.
ALIAS_ID = "0x982667c59ade8ab8a99188f4de38c68b97fc2ca7ba28a1e9d8d683996247e152"
WALLET_PASSWORD = "some_hopefully_secure_password"

wallet = Wallet("./alice-database")

account = wallet.get_account("Alice")

# Sync account with the node
response = account.sync()

wallet.set_stronghold_password(WALLET_PASSWORD)

# Create burn object.
to_burn = Burn().add_alias(ALIAS_ID)

# Send transaction.
transaction = account.burn(to_burn)
print(transaction)
