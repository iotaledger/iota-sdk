from iota_sdk import MnemonicSecretManager, CoinType, SecretManager
from dotenv import load_dotenv
import os

load_dotenv()

if 'MNEMONIC' not in os.environ:
    raise Exception(".env MNEMONIC is undefined, see .env.example")

# In this example we will create addresses from a mnemonic

secret_manager = SecretManager(MnemonicSecretManager(os.environ['MNEMONIC']))

# Generate public address with default account index and range.
addresses = secret_manager.generate_ed25519_addresses()

print('List of generated public addresses:', *addresses, sep='\n')
print()

addresses = secret_manager.generate_ed25519_addresses(
    coin_type=CoinType.SHIMMER,
    account_index=0,
    start=0,
    end=4,
    internal=False,
    bech32_hrp='rms')

print('List of generated public addresses:', *addresses, sep='\n')
print()
