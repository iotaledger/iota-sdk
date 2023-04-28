from iota_sdk import Wallet

# In this example we will burn an NFT

# Explorer url
EXPLORER = "https://explorer.shimmer.network/testnet"

wallet = Wallet('./alice-database')

account = wallet.get_account('Alice')

# Sync account with the node
response = account.sync()
print(f'Synced: {response}')

wallet.set_stronghold_password("some_hopefully_secure_password")

# TODO: replace with your own values.
nftId = "0xf95f4d5344217a2ba19a6c19a47f97d267edf8c4d76a7b8c08072ad35acbebbe"

# Send transaction.
transaction = account.burn_nft(nftId)

print(f'Transaction: {transaction["transactionId"]}')
print(f'Block sent: {EXPLORER}/block/" + {transaction["blockId"]}');
