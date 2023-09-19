import os

from dotenv import load_dotenv

from iota_sdk import Wallet, FilterOptions, Utils, UnlockConditionType, StateControllerAddressUnlockCondition

load_dotenv()

# In this example we will update the state controller of an account output.

if 'WALLET_DB_PATH' not in os.environ:
    raise Exception(".env WALLET_DB_PATH is undefined, see .env.example")

if 'STRONGHOLD_PASSWORD' not in os.environ:
    raise Exception(".env STRONGHOLD_PASSWORD is undefined, see .env.example")

if 'EXPLORER_URL' not in os.environ:
    raise Exception(".env EXPLORER_URL is undefined, see .env.example")

wallet = Wallet(os.environ['WALLET_DB_PATH'])

account = wallet.get_account('Alice')

# Sync account with the node
balance = account.sync()

if len(balance.accounts) == 0:
    raise Exception("No Account available in account 'Alice'")

account_id = balance.accounts[0]

account_output_data = account.unspent_outputs(
    FilterOptions(accountIds=[account_id]))[0]
print(
    f"Account {account_id} found in unspent output: {account_output_data.outputId}")

wallet.set_stronghold_password(os.environ["STRONGHOLD_PASSWORD"])

print(f"{ account.generate_ed25519_addresses(1)[0].address}")
new_state_controller = Utils.parse_bech32_address(
    account.generate_ed25519_addresses(1)[0].address)
print(f"{new_state_controller.__dict__}")

account_output = account_output_data.output


def update_state_controller(unlock_condition):
    """Update unlock condition if its a state controlled type."""
    if unlock_condition.type == UnlockConditionType.StateControllerAddress:
        return StateControllerAddressUnlockCondition(new_state_controller)
    return unlock_condition


updated_unlock_conditions = list(map(
    update_state_controller, account_output.unlockConditions))
updated_account_output = wallet.get_client().build_account_output(
    account_id,
    unlock_conditions=updated_unlock_conditions,
    state_index=account_output.stateIndex,
    state_metadata=account_output.stateMetadata,
    foundry_counter=account_output.foundryCounter,
    immutable_features=account_output.immutableFeatures,
    features=account_output.features,
)

print('Sending transaction...')
transaction = account.send_outputs([updated_account_output])
print(f'Transaction sent: {transaction.transactionId}')

# Wait for transaction to get included
blockId = account.reissue_transaction_until_included(
    transaction.transactionId)
print(f'Block included: {os.environ["EXPLORER_URL"]}/block/{blockId}')
