from iota_wallet import IotaWallet

# In this example we will destroy an alias output

# TODO: replace with your own values.
ALIAS_ID = "0x08429fe5864378ce70699fc2d22bb144cb86a3c4833d136e3b95c5dadfd6ba0cef0500000000"
WALLET_PASSWORD = "some_hopefully_secure_password"

wallet = IotaWallet('./alice-database')

account = wallet.get_account('Alice')

# Sync account with the node
response = account.sync()

wallet.set_stronghold_password(WALLET_PASSWORD)

# Send transaction.
transaction = account.destroy_alias(ALIAS_ID)
print(transaction)
