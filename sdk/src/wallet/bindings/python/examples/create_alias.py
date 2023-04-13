from iota_wallet import IotaWallet

# In this example we will create an alias ouput

# TODO: replace with your own values.
WALLET_PASSWORD = "some_hopefully_secure_password"

wallet = IotaWallet('./alice-database')

account = wallet.get_account('Alice')

# Sync account with the node
account.sync()

wallet.set_stronghold_password(WALLET_PASSWORD)

# Send transaction.
transaction = account.create_alias_output(None, None)
print(transaction)
