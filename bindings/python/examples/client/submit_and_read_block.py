#################################################
# Example script for tagged data payloads
#################################################
# This script has three steps:
#  1) Prepare the payload for a block
#  2) Submit the block to the Shimmer test network
#  3) Use the block ID to read the payload back from the network


# Import the python iota client
# Make sure you have first installed it with `pip install iota_sdk`
from iota_sdk import Client, hex_to_utf8, utf8_to_hex, TaggedDataPayload
from dotenv import load_dotenv
import os

load_dotenv()

node_url = os.environ.get('NODE_URL', 'https://api.testnet.shimmer.network')

# Create a Client instance
client = Client(nodes=[node_url])


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
tag_hex = utf8_to_hex(tag)
message_hex = utf8_to_hex(message)

print('\nYour prepared block content is:')
print(f'  tag: {tag}')
print(f'  tag converted to hex: {tag_hex}')
print(f'  message: {message}')
print(f'  message converted to hex: {message_hex}')


########################################################
# Step 2: Submit your block to the Shimmer test network
########################################################
# A block must be built, to which the payload is attached.
# The submit_payload function handles this task.

# Create and post a block with a tagged data payload
# The client returns your block data (blockIdAndBlock)
blockIdAndBlock = client.submit_payload(
    TaggedDataPayload(
        utf8_to_hex("tag"),
        utf8_to_hex("data")))

block_id = blockIdAndBlock[0]
block = blockIdAndBlock[1]

print('\nThe block ID for your submitted block is:')
print(f'  {block_id}')

print('\nMetadata for your submitted block is:')
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
payload_out = block.payload
tag_hex_out = block.payload.tag
message_hex_out = block.payload.data

# Unpackage the payload (from hex to text)
message_out = hex_to_utf8(message_hex_out)
print('\nYour message, read from the Shimmer network:')
print(f'  {message_out}')

# Or see the message online, with the testnet explorer.
print(
    f'\nOr see the message with the testnet explorer: {os.environ["EXPLORER_URL"]}/block/{block_id}')
