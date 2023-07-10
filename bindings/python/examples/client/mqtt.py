from iota_sdk import Client, init_logger
from dotenv import load_dotenv
import os
import json

load_dotenv()

node_url = os.environ.get('NODE_URL', 'https://api.testnet.shimmer.network')

# Create a Client instance
client = Client(nodes=[node_url])

def callback(event):
    event_dict = json.loads(event)
    print(event_dict)

client.listen(["blocks"], callback)

import time
time.sleep(10)