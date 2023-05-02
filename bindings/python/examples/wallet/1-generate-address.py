from iota_sdk import Wallet

# This example generates a new address.

wallet = Wallet('./alice-database')

wallet.set_stronghold_password("some_hopefully_secure_password")

account = wallet.get_account('Alice')

address = account.generate_addresses(1)
# address = account.generate_addresses(
#     1, {'internal': True, 'metadata': {'syncing': True, 'network': 'Testnet'}})
print(f'Address: {address}')
