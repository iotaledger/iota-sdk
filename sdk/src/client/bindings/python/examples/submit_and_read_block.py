#################################################
# Example script for tagged data payloads
#################################################
# This script has three steps:
#  1) Prepare the payload for a block
#  2) Submit the block to the Shimmer test network
#  3) Use the block ID to read the payload back from the network

# Import the python iota client
# Make sure you have first installed it with `pip install iota_client`
from iota_client import IotaClient

# Create an IotaClient instance
client = IotaClient({'nodes': ['https://api.testnet.shimmer.network']})


########################################################
# Step 1: Prepare the data in hex format for your block
########################################################
# Data is submitted to the Shimmer network as a block.
# This block can contain a 'payload' with data.
# This payload has a 'tag' and the 'data' itself, both in hex format.
# The Shimmer network requires a "0x" at the beginning of hex strings.

# Write a tag and message
tag = "Hello"
# message = "Hello again. You can use one line or multiple lines!"
message = """
I am a
robot!
"""

# Convert to hex
# 1) Ensure the string is in UTF-8 using `encode()`.
# 2) Convert to hex using `hex()`.
# 3) Prepend the `0x` required by Shimmer.
tag_hex = "0x"+tag.encode("utf-8").hex()
message_hex = "0x"+message.encode("utf-8").hex()

print(f'\nYour prepared block content is:')
print(f'  tag: {tag_hex}')
print(f'  tag converted to hex: {tag_hex}')
print(f'  message: {message}')
print(f'  message converted to hex: {message_hex}')

# Collect your hex data into a dictionary. This will be the payload
payload = {"tag": tag_hex, "data": message_hex}


########################################################
# Step 2: Submit your block to the Shimmer test network
########################################################
# A block must be built, to which the payload is attached.
# The build_and_post_block function handles this task.

# Create and post a block with a tagged data payload
# The client returns your block data (blockIdAndBlock)
blockIdAndBlock = client.build_and_post_block(secret_manager=None, options=payload)

block_id = blockIdAndBlock[0]
block = blockIdAndBlock[1]

print(f'\nThe block ID for your submitted block is:')
print(f'  {block_id}')

print(f'\nMetadata for your submitted block is:')
print(f'  {block}')

########################################################
# Step 3: Use the block ID to read the payload back
########################################################
# The network can be queried using the block ID.
# There are two methods to query the network.
#   get_block_metadata - metadata only
#   get_block_data - metadata and payload

# Get the metadata for the block
metadata = client.get_block_metadata(block_id)

# Get the whole block
block = client.get_block_data(block_id)
payload_out = block['payload']
tag_hex_out = block['payload']['tag']
message_hex_out = block['payload']['data']

# Unpackage the payload (from hex to text)
# Remember to remove the '0x' from the hex
message_out = bytes.fromhex(message_hex_out[2:]).decode('utf-8')
print(f'\nYour message, read from the Shimmer network:')
print(f'  {message_out}')

# Or see the message online, with the testnet explorer
explorer_url = 'https://explorer.iota.org/testnet/block/'+block_id
print(f'\nOr see the message with the testnet explorer:')
print(f'  {explorer_url}')