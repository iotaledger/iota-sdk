from iota_wallet import IotaWallet

# In this example we will mint an nft

# Explorer url
EXPLORER = "https://explorer.shimmer.network/testnet"

wallet = IotaWallet('./alice-database')

account = wallet.get_account('Alice')

# Sync account with the node
response = account.sync()
print(f'Synced: {response}')

wallet.set_stronghold_password("some_hopefully_secure_password")

outputs = [{
    "immutableMetadata": "0x"+"some immutable nft metadata".encode("utf-8").hex(),
}]

transaction = account.mint_nfts(outputs)

print(f'Transaction: {transaction.transaction_id}')
print(f'Block sent: {EXPLORER}/block/" + {transaction.block_id});
