import iota_sdk
from iota_sdk import Wallet, StrongholdSecretManager, CoinType

v2_path = "../../sdk/tests/wallet/fixtures/v2.stronghold"
v3_path = "./v3.stronghold"
client_options = {
    'nodes': ['https://api.testnet.shimmer.network'],
}
coin_type = CoinType.SHIMMER

try:
    secret_manager = StrongholdSecretManager(v2_path, "current_password")
    # This should fail with error, migration required.
    wallet = Wallet('./alice-database', client_options, coin_type, secret_manager)
except ValueError as e:
    print(e)

iota_sdk.migrate_stronghold_snapshot_v2_to_v3(v2_path, "current_password", "wallet.rs", 100, v3_path, "new_password")

secret_manager = StrongholdSecretManager(v3_path, "new_password")
# This shouldn't fail anymore as snapshot has been migrated.
wallet = Wallet('./alice-database', client_options, coin_type, secret_manager)

account = wallet.create_account('Alice')

print(account['publicAddresses'])