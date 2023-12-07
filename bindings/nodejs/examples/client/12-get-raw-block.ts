// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Client, initLogger } from '@iota/sdk';
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./client/12-get-raw-block.ts

// In this example we will get the raw bytes of a block.
async function run() {
    initLogger();
    for (const envVar of ['NODE_URL']) {
        if (!(envVar in process.env)) {
            throw new Error(`.env ${envVar} is undefined, see .env.example`);
        }
    }

    const client = new Client({
        // Insert your node URL in the .env.
        nodes: [process.env.NODE_URL as string],
    });

    try {
        // Get a random block ID.
        const blockId = (await client.getTips())[0];

        const rawBytes = await client.getBlockRaw(blockId);
        console.log('Block bytes: ', rawBytes);
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
