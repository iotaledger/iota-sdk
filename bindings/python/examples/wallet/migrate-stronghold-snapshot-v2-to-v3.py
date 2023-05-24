import iota_sdk
from iota_sdk import Wallet, StrongholdSecretManager, CoinType

client_options = {
    'nodes': ['https://api.testnet.shimmer.network'],
}

coin_type = CoinType.SHIMMER

secret_manager = StrongholdSecretManager("examples/wallet/fixtures/v2.stronghold", "current_password")

try:
   wallet = Wallet('./alice-database', client_options, coin_type, secret_manager)
except ValueError as e:
    print(e)

iota_sdk.migrate_stronghold_snapshot_v2_to_v3("examples/wallet/fixtures/v2.stronghold", "current_password", "wallet.rs", 100, "examples/wallet/fixtures/v3.stronghold", "new_password")

secret_manager = StrongholdSecretManager("examples/wallet/fixtures/v3.stronghold", "new_password")

wallet = Wallet('./alice-database', client_options, coin_type, secret_manager)

account = wallet.create_account('Alice')

print(account['publicAddresses'])