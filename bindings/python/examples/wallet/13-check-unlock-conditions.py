import os

from dotenv import load_dotenv
from iota_sdk import OutputParams, Utils, Wallet, WalletOptions

load_dotenv()

# In this example we check if an output has only an address unlock
# condition and that the address is from the wallet.

wallet = Wallet(WalletOptions(storage_path=os.environ.get('WALLET_DB_PATH')))
address = wallet.address()

# using prepare_output
output = wallet.prepare_output(OutputParams(
    address, 1000000))

walletAddress = Utils.parse_bech32_address(address)

controlled_by_wallet = False

if len(
        output.unlock_conditions) == 1 and output.unlock_conditions[0].type == 0:
    if output.unlock_conditions[0].address.pub_key_hash == walletAddress.pub_key_hash:
        controlled_by_wallet = True

print(
    f'The output has only an address unlock condition and the address is from the wallet: {controlled_by_wallet}')
