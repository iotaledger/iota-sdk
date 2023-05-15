from iota_sdk import Wallet

# This example lists all accounts in the wallet.

wallet = Wallet('./alice-database')

for account in wallet.get_accounts():
    print(account['alias'])
