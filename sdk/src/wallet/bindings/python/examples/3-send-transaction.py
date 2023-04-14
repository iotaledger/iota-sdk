from iota_wallet import IotaWallet

# This example sends a transaction.

# Explorer url
EXPLORER = "https://explorer.shimmer.network/testnet"

wallet = IotaWallet('./alice-database')

account = wallet.get_account('Alice')

# Sync account with the node
response = account.sync()
print(f'Synced: {response}')

wallet.set_stronghold_password("some_hopefully_secure_password")

outputs = [{
    "address": "rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu",
    "amount": "1000000",
}]

transaction = account.send_amount(outputs)

print(f'Transaction: {transaction.transaction_id}')
print(f'Block sent: {EXPLORER}/block/" + {transaction.block_id});