from iota_sdk import Wallet, utf8_to_hex

# In this example we will mint an nft

# Explorer url
EXPLORER = "https://explorer.shimmer.network/testnet"

wallet = Wallet('./alice-database')

account = wallet.get_account('Alice')

# Sync account with the node
response = account.sync()
print(f'Synced: {response}')

wallet.set_stronghold_password("some_hopefully_secure_password")

outputs = [{
    "immutableMetadata": utf8_to_hex("some immutable nft metadata"),
}]

transaction = account.mint_nfts(outputs)

print(f'Transaction: {transaction["transactionId"]}')
print(f'Block sent: {EXPLORER}/block/" + {transaction["blockId"]}');
