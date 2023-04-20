from iota_sdk import Client, utf8_to_hex

# Create a Client instance
client = Client(nodes=['https://api.testnet.shimmer.network'])

# Create and post a block with a tagged data payload
block = client.build_and_post_block(
    tag=utf8_to_hex('hello'), data=utf8_to_hex('hello'))
print(f'{block}')
