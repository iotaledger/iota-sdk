// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    BasicBlock,
    BlockType,
    Client,
    initLogger,
    parseBlockWrapper,
} from '@iota/sdk';

require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./client/10-mqtt.ts

// In this example we will listen to MQTT topics and print the block payloads.
async function run() {
    initLogger();
    if (!process.env.NODE_URL) {
        throw new Error('.env NODE_URL is undefined, see .env.example');
    }

    // Connecting to a MQTT broker using raw ip doesn't work with TCP. This is a limitation of rustls.
    const client = new Client({
        // Insert your node URL in the .env.
        nodes: [process.env.NODE_URL],
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
            const block = parseBlockWrapper(JSON.parse(parsed.payload));

            if (block.type === BlockType.Basic) {
                const basic = block as BasicBlock;
                console.log('payload:', basic.block.payload);
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

run();
