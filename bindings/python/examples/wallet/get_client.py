from iota_sdk import Wallet

# This example gets a client from the wallet.

wallet = Wallet('./alice-database')

client = wallet.get_client()

info = client.get_info()
print(f'{info}')
