from iota_sdk import Wallet

# This example lists all addresses in the account.

wallet = Wallet('./alice-database')

account = wallet.get_account('Alice')

addresses = account.addresses()

for address in addresses:
    print(address['address'])
