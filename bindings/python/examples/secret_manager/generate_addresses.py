from iota_sdk import MnemonicSecretManager, CoinType, SecretManager
from dotenv import load_dotenv
import os

load_dotenv()

if 'NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1' not in os.environ:
    print(".env mnemonic is undefined, see .env.example")
    sys.exit(1)

# In this example we will create addresses from a mnemonic

secret_manager = SecretManager(MnemonicSecretManager(os.environ['NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1']))

# Generate public address with default account index and range.
addresses = secret_manager.generate_addresses()

print('List of generated public addresses:', *addresses, sep='\n')
print()

addresses = secret_manager.generate_addresses( 
    coin_type=CoinType.SHIMMER,
    account_index=0,
    start=0,
    end=4,
    internal=False,
    bech32_hrp='rms')

print('List of generated public addresses:', *addresses, sep='\n')
print()
