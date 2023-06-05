from iota_sdk import Client, MnemonicSecretManager, CoinType

from dotenv import load_dotenv
import os

load_dotenv()

node_url = os.environ.get('NODE_URL', 'https://api.testnet.shimmer.network')

# Create a Client instance
client = Client(nodes=[node_url])

if 'NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1' not in os.environ:
    raise Exception(".env mnemonic is undefined, see .env.example")

# In this example we will create addresses from a mnemonic
secret_manager = MnemonicSecretManager(
    os.environ['NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1'])

# Generate public address with default account index and range.
addresses = client.generate_ed25519_addresses(secret_manager)

print('List of generated public addresses:', *addresses, sep='\n')
print()

# Generate public address with custom account index and range.
addresses = client.generate_ed25519_addresses(secret_manager,
                                      account_index=0,
                                      start=0,
                                      end=4)

print('List of generated public addresses:', *addresses, sep='\n')
print()

# Generate internal addresses with custom account index and range.
addresses = client.generate_ed25519_addresses(secret_manager,
                                      account_index=0,
                                      start=0,
                                      end=4,
                                      internal=True)

print('List of generated internal addresses:', *addresses, sep='\n')
print()

# Generate addresses with providing all inputs, that way it can also be done offline without a node.
addresses = client.generate_ed25519_addresses(secret_manager,
                                      coin_type=CoinType.SHIMMER,
                                      account_index=0,
                                      start=0,
                                      end=4,
                                      internal=False,
                                      # Generating addresses with client.generateEd25519Addresses(secretManager, options={}), will by default get the bech32_hrp (Bech32
                                      # human readable part) from the node info, generating it "offline" requires setting it in the generateAddressesOptions
                                      bech32_hrp='rms')

print('List of offline generated public addresses:', *addresses, sep='\n')
print()
