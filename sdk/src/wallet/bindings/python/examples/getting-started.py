from iota_wallet import IotaWallet, StrongholdSecretManager

NODE_URL = "https://api.testnet.shimmer.network"
STORAGE_PATH = "walletdb"
STRONGHOLD_SNAPSHOT_PATH = "vault.stronghold"
SHIMMER_COIN_TYPE = 4219

# Change to a secure password.
password = "some-secure-password"

# Setup Stronghold secret manager
secret_manager = StrongholdSecretManager(STRONGHOLD_SNAPSHOT_PATH, password)

client_options = {
    'nodes': [NODE_URL],
}

# Set up and store the wallet.
wallet = IotaWallet(STORAGE_PATH, client_options, SHIMMER_COIN_TYPE, secret_manager)

# Generate a mnemonic and store it in the Stronghold vault.
mnemonic = wallet.generate_mnemonic()
wallet.store_mnemonic(mnemonic)

# Create an account and get the first address.
wallet.create_account('Alice')
account = wallet.get_account('Alice')

address = account.addresses()[0]

# Print the account data.
print(f'Mnemonic:\n{mnemonic}\n')
print(f'Address:\n{address["address"]}\n')
