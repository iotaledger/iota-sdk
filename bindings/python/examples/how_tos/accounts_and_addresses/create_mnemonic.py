from iota_sdk import Utils

# Generate a random BIP39 mnemonic
mnemonic = Utils.generate_mnemonic()
print(f'Mnemonic: {mnemonic}')
