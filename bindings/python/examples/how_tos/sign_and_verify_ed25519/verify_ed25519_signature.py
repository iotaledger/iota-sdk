from iota_sdk import Utils, utf8_to_hex

# In this example we will verify an Ed25519 signature.

FOUNDRY_METADATA = '{"standard":"IRC30","name":"NativeToken","description":"A native token","symbol":"NT","decimals":6,"logoUrl":"https://my.website/nativeToken.png"}'
PUBLIC_KEY = "0x67b7fc3f78763c9394fc4fcdb52cf3a973b6e064bdc3defb40a6cb2c880e6f5c"
ED25519_SIGNATURE = "0x5437ee671f182507103c6ae2f6649083475019f2cc372e674be164577dd123edd7a76291ba88732bbe1fae39688b50a3678bce05c9ef32c9494b3968f4f07a01"

bech32_address = Utils.hex_public_key_to_bech32_address(PUBLIC_KEY, 'rms')
address = Utils.parse_bech32_address(bech32_address)

message = utf8_to_hex(FOUNDRY_METADATA)
validSignature = Utils.verify_ed25519_signature(
    {"type": 0, "publicKey": PUBLIC_KEY, "signature": ED25519_SIGNATURE},
    message,
    address,
)
print(f'Valid signature: {validSignature}')
