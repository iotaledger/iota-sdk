// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Client, initLogger } from '@iota/sdk';
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./how_tos/client/get-health.ts

// In this example we will get the node health
async function run() {
    initLogger();
    if (!process.env.NODE_URL) {
        throw new Error('.env NODE_URL is undefined, see .env.example');
    }

    const client = new Client({
        // Insert your node URL in the .env.
        nodes: [process.env.NODE_URL],
        localPow: true,
    });

    try {
        const isHealthy = await client.getHealth(process.env.NODE_URL);
        console.log('Healthy: ', isHealthy);
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
