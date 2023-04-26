import iota_wallet
from iota_wallet import IotaWallet, StrongholdSecretManager

client_options = {
    'nodes': ['https://api.testnet.shimmer.network'],
}

coin_type = 4219

secret_manager = StrongholdSecretManager("examples/fixtures/v2.stronghold", "current_password")

try:
   wallet = IotaWallet('./alice-database', client_options, coin_type, secret_manager)
except ValueError as e:
    print(e)

iota_wallet.migrate_stronghold_snapshot_v2_to_v3("examples/fixtures/v2.stronghold", "current_password", "examples/fixtures/v3.stronghold", "new_password")

secret_manager = StrongholdSecretManager("examples/fixtures/v3.stronghold", "new_password")

wallet = IotaWallet('./alice-database', client_options, coin_type, secret_manager)

account = wallet.create_account('Alice')

print(account['publicAddresses'])