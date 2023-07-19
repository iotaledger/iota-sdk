# Copyright 2023 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

# This example shows how to listen to MQTT events of a node.

from iota_sdk import Client
from dotenv import load_dotenv
import os
import threading
import json

load_dotenv()

node_url = os.environ.get('NODE_URL', 'https://api.testnet.shimmer.network')

# Create a Client instance
client = Client(nodes=[node_url])

received_events = 0

received_10_events = threading.Event()


def callback(event):
    event_dict = json.loads(event)
    print(event_dict)
    global received_events
    received_events += 1
    if received_events > 10:
        received_10_events.set()


# Topics can be found here
# https://studio.asyncapi.com/?url=https://raw.githubusercontent.com/iotaledger/tips/main/tips/TIP-0028/event-api.yml
client.listen_mqtt(["blocks"], callback)

# Exit after 10 received events
received_10_events.wait()
client.clear_mqtt_listeners(["blocks"])
