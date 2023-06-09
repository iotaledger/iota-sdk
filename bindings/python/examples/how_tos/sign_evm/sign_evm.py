from iota_sdk import Client, StrongholdSecretManager, SecretManager, HD_WALLET_TYPE, HARDEN_MASK, CoinType, Utils, utf8_to_hex
from dotenv import load_dotenv
import os

load_dotenv()

# In this example we will sign with secp256k1.

FOUNDRY_METADATA = '{"standard":"IRC30","name":"NativeToken","description":"A native token","symbol":"NT","decimals":6,"logoUrl":"https://my.website/nativeToken.png"}'
ACCOUNT_INDEX = 0
INTERNAL_ADDRESS = False
ADDRESS_INDEX = 0

if 'NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1' not in os.environ:
    raise Exception(".env mnemonic is undefined, see .env.example")

if 'STRONGHOLD_PASSWORD' not in os.environ:
    raise Exception(".env STRONGHOLD_PASSWORD is undefined, see .env.example")

secret_manager = SecretManager(StrongholdSecretManager(
    "sign_evm.stronghold", os.environ['STRONGHOLD_PASSWORD']))

# Store the mnemonic in the Stronghold snapshot, this needs to be done only the first time.
# The mnemonic can't be retrieved from the Stronghold file, so make a backup in a secure place!
secret_manager.store_mnemonic(
    os.environ['NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1'])

bip32_chain = [
    HD_WALLET_TYPE | HARDEN_MASK,
    CoinType.ETHER | HARDEN_MASK,
    ACCOUNT_INDEX | HARDEN_MASK,
    1 if INTERNAL_ADDRESS else 0,
    ADDRESS_INDEX,
]

message = utf8_to_hex(FOUNDRY_METADATA)
evm_signature = secret_manager.sign_evm(message, bip32_chain)
print(f'Public key: {evm_signature["publicKey"]}')
print(f'Signature: {evm_signature["signature"]}')
