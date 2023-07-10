from iota_sdk import Client, init_logger
from dotenv import load_dotenv
import os
import json

load_dotenv()

node_url = os.environ.get('NODE_URL', 'https://api.testnet.shimmer.network')

# Create a Client instance
client = Client(nodes=[node_url])

received_events = 0


def callback(event):
    event_dict = json.loads(event)
    print(event_dict)
    global received_events
    received_events += 1


# Topics can be found here https://studio.asyncapi.com/?url=https://raw.githubusercontent.com/iotaledger/tips/main/tips/TIP-0028/event-api.yml
client.listen_mqtt(["blocks"], callback)

# Exit after 10 received events
while True:
    if received_events > 10:
        client.clear_mqtt_listeners(["blocks"])
        exit()
