from iota_sdk import Client, StrongholdSecretManager, SecretManager, Bip44, CoinType, Utils, utf8_to_hex
from dotenv import load_dotenv
import os

load_dotenv()

# In this example we will sign with secp256k1.

FOUNDRY_METADATA = '{"standard":"IRC30","name":"NativeToken","description":"A native token","symbol":"NT","decimals":6,"logoUrl":"https://my.website/nativeToken.png"}'
ACCOUNT_INDEX = 0
INTERNAL_ADDRESS = False
ADDRESS_INDEX = 0

if 'MNEMONIC' not in os.environ:
    raise Exception(".env MNEMONIC is undefined, see .env.example")

if 'STRONGHOLD_PASSWORD' not in os.environ:
    raise Exception(".env STRONGHOLD_PASSWORD is undefined, see .env.example")

secret_manager = SecretManager(StrongholdSecretManager(
    "sign_secp256k1_ecdsa.stronghold", os.environ['STRONGHOLD_PASSWORD']))

# Store the mnemonic in the Stronghold snapshot, this needs to be done only the first time.
# The mnemonic can't be retrieved from the Stronghold file, so make a backup in a secure place!
secret_manager.store_mnemonic(os.environ['MNEMONIC'])

bip44_chain = Bip44(
    CoinType.ETHER,
    ACCOUNT_INDEX,
    1 if INTERNAL_ADDRESS else 0,
    ADDRESS_INDEX,
)

message = utf8_to_hex(FOUNDRY_METADATA)
secp256k1_ecdsa_signature = secret_manager.sign_secp256k1_ecdsa(message, bip44_chain)
print(f'Public key: {secp256k1_ecdsa_signature["publicKey"]}')
print(f'Signature: {secp256k1_ecdsa_signature["signature"]}')
