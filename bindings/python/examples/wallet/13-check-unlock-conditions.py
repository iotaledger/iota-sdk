import os

from dotenv import load_dotenv

from iota_sdk import OutputParams, Utils, Wallet

load_dotenv()

# In this example we check if an output has only an address unlock
# condition and that the address is from the account.

wallet = Wallet(os.environ['WALLET_DB_PATH'])

account = wallet.get_account("Alice")

accountAddresses = account.addresses()

# using prepare_output
output = account.prepare_output(OutputParams(
    accountAddresses[0].address, "1000000"))


def hexAddress(address):
    """Converts an address to hex"""
    return Utils.bech32_to_hex(address.address)


hexEncodedAccountAddresses = map(hexAddress, accountAddresses)

controlled_by_account = False

if len(
        output.unlock_conditions) == 1 and output.unlock_conditions[0].type == 0:
    if output.unlock_conditions[0].address.pub_key_hash in hexEncodedAccountAddresses:
        controlled_by_account = True

print(
    f'The output has only an address unlock condition and the address is from the account: {controlled_by_account}')
