from iota_sdk import MnemonicSecretManager, CoinType, SecretManager

# In this example we will create addresses from a mnemonic

secret_manager = SecretManager(MnemonicSecretManager("endorse answer radar about source reunion marriage tag sausage weekend frost daring base attack because joke dream slender leisure group reason prepare broken river"))

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
