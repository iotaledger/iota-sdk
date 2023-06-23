// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    Block,
    Client,
    initLogger,
    MilestonePayload,
    parsePayload,
} from '@iota/sdk';
import { plainToInstance } from 'class-transformer';

require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./client/10-mqtt.ts

// Initialize MQTT listener
async function run() {
    initLogger();
    if (!process.env.NODE_URL) {
        throw new Error('.env NODE_URL is undefined, see .env.example');
    }

    // Connecting to a MQTT broker using raw ip doesn't work with TCP. This is a limitation of rustls.
    const client = new Client({
        nodes: [process.env.NODE_URL],
    });

    // Array of topics to subscribe to
    // Topics can be found here https://studio.asyncapi.com/?url=https://raw.githubusercontent.com/iotaledger/tips/stardust-event-api/tips/TIP-0028/event-api.yml
    const topics = ['blocks'];

    const callback = function (error: Error, data: string) {
        if (error != null) {
            console.log(error);
            return;
        }

        let parsed = JSON.parse(data);
        if (parsed.topic == 'milestone') {
            let payload = parsePayload(
                JSON.parse(parsed.payload),
            ) as MilestonePayload;
            let index = payload.index;
            let previousMilestone = payload.previousMilestoneId;
            console.log(
                'New milestone index' +
                    index +
                    ', previous ID: ' +
                    previousMilestone,
            );
        } else if (parsed.topic == 'blocks') {
            let block = plainToInstance(Block, JSON.parse(parsed.payload));
            console.log('payload:', block.payload);
        }
    };

    await client.listen(topics, callback);

    // Clear listener after 10 seconds
    setTimeout(async () => {
        await client.clearListeners(topics);
        console.log('Listener cleared');
        // Exit the process
        setTimeout(async () => process.exit(0), 2000);
    }, 10000);
}

run();
