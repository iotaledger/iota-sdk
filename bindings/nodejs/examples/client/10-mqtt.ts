// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Client, initLogger, parseBlock } from '@iota/sdk';

require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./client/10-mqtt.ts

// In this example we will listen to MQTT topics and print the block payloads.
async function run() {
    initLogger();
    for (const envVar of ['NODE_URL']) {
        if (!(envVar in process.env)) {
            throw new Error(`.env ${envVar} is undefined, see .env.example`);
        }
    }

    // Connecting to a MQTT broker using raw ip doesn't work with TCP. This is a limitation of rustls.
    const client = await Client.create({
        // Insert your node URL in the .env.
        nodes: [process.env.NODE_URL as string],
    });

    // Array of topics to subscribe to
    // Topics can be found here https://studio.asyncapi.com/?url=https://raw.githubusercontent.com/iotaledger/tips/main/tips/TIP-0028/event-api.yml
    const topics = ['blocks'];

    const callback = function (error: Error, data: string) {
        if (error != null) {
            console.log(error);
            return;
        }

        const parsed = JSON.parse(data);
        if (parsed.topic == 'blocks') {
            const block = parseBlock(JSON.parse(parsed.payload));

            if (block.isBasic()) {
                const basic = block.asBasic();
                console.log('payload:', basic.payload);
            }
        }
    };

    await client.listenMqtt(topics, callback);

    // Clear listener after 10 seconds
    setTimeout(async () => {
        await client.clearMqttListeners(topics);
        console.log('Listener cleared');
        // Exit the process
        setTimeout(async () => process.exit(0), 2000);
    }, 10000);
}

void run().then(() => process.exit());
