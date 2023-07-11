from iota_sdk import Client, StrongholdSecretManager, SecretManager, Bip44, CoinType, Utils, utf8_to_hex
from dotenv import load_dotenv
import os

load_dotenv()

# In this example we will sign with Ed25519.

FOUNDRY_METADATA = '{"standard":"IRC30","name":"NativeToken","description":"A native token","symbol":"NT","decimals":6,"logoUrl":"https://my.website/nativeToken.png"}'
ACCOUNT_INDEX = 0
INTERNAL_ADDRESS = False
ADDRESS_INDEX = 0

if 'MNEMONIC' not in os.environ:
    raise Exception(".env MNEMONIC is undefined, see .env.example")

if 'STRONGHOLD_PASSWORD' not in os.environ:
    raise Exception(".env STRONGHOLD_PASSWORD is undefined, see .env.example")

secret_manager = SecretManager(StrongholdSecretManager(
    "sign_ed25519.stronghold", os.environ['STRONGHOLD_PASSWORD']))

# Store the mnemonic in the Stronghold snapshot, this needs to be done only the first time.
# The mnemonic can't be retrieved from the Stronghold file, so make a backup in a secure place!
secret_manager.store_mnemonic(
    os.environ['MNEMONIC'])

bip44_chain = Bip44(
    CoinType.SHIMMER,
    ACCOUNT_INDEX,
    1 if INTERNAL_ADDRESS else 0,
    ADDRESS_INDEX,
)

message = utf8_to_hex(FOUNDRY_METADATA)
ed25519_signature = secret_manager.sign_ed25519(message, bip44_chain)
print(
    f'Public key: {ed25519_signature.publicKey}\nSignature: {ed25519_signature.signature}')

bech32_address = Utils.hex_public_key_to_bech32_address(
    ed25519_signature.publicKey, "rms")
print(f'Address: {bech32_address}')
