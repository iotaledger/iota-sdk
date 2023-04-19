from iota_sdk import Client, StrongholdSecretManager, SecretManager

# Create a Client instance
client = Client(nodes = ['https://api.testnet.shimmer.network'])

secret_manager = StrongholdSecretManager("client.stronghold", "some_hopefully_secure_password")

# Store the mnemonic in the Stronghold snapshot, this needs to be done only the first time.
# The mnemonic can't be retrieved from the Stronghold file, so make a backup in a secure place!
result = SecretManager(secret_manager).store_mnemonic("flame fever pig forward exact dash body idea link scrub tennis minute " +
    "surge unaware prosper over waste kitten ceiling human knife arch situate civil")

# Generate public address with custom account index and range.
address = client.generate_addresses(secret_manager, account_index=0, start=0, end=1)

print(f'Address: {address[0]}')
