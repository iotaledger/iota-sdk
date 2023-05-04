from iota_sdk import Wallet
import time

# In this example we will mint native tokens

# Explorer url
EXPLORER = "https://explorer.shimmer.network/testnet"

wallet = Wallet('./alice-database')

account = wallet.get_account('Alice')

# Sync account with the node
response = account.sync()
print(f'Synced: {response}')

wallet.set_stronghold_password("some_hopefully_secure_password")

transaction = account.create_alias_output(None, None)

# Wait a few seconds for the transaction to get confirmed
time.sleep(7)

account.sync()

params = {
    # 1000 hex encoded
    "circulatingSupply": "0x3e8",
    "maximumSupply": "0x3e8",
    "foundryMetadata": "0xab",
}

transaction = account.mint_native_token(params, None)

print(f'Transaction: {transaction["transactionId"]}')
print(f'Block sent: {EXPLORER}/block/" + {transaction["blockId"]}');
