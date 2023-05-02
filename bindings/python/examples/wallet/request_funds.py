from iota_sdk import Wallet, Client

import time

# This example requests funds from the faucet

wallet = Wallet('./alice-database')

account = wallet.get_account('Alice')

# Sync account with the node
response = account.sync()
print(f'Synced: {response}')

# Balance before funding
balance = account.get_balance()
print(
    f'balance before faucet request: { balance[ "baseCoin" ][ "available" ] }')

response = Client().request_funds_from_faucet("https://faucet.testnet.shimmer.network/api/enqueue",
                                              "rms1qpszqzadsym6wpppd6z037dvlejmjuke7s24hm95s9fg9vpua7vluaw60xu")
print(response)

time.sleep(20)

# Sync account with the node
response = account.sync()

# Balance after funding
balance = account.get_balance()
print(
    f'balance after faucet request: { balance[ "baseCoin" ][ "available" ] }')
