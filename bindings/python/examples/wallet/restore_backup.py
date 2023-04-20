from iota_sdk import Wallet

# This example restores the wallet from a stronghold.

client_options = {
    'nodes': ['https://api.testnet.shimmer.network'],
}

# Shimmer coin type
coin_type = 4219

wallet = Wallet('./restore-backup-database', client_options,
                    coin_type, 'Placeholder')


wallet.restore_backup("backup.stronghold", "some_hopefully_secure_password")

accounts = wallet.get_accounts()
print(f'Restored accounts: {accounts}')