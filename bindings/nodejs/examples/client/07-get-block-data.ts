// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Client, initLogger } from '@iota/sdk';
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./client/07-get-block-data.ts

// In this example we will send a block and get the data and metadata for it.
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
        // Fetch a block ID from the node.
        const blockIds = await client.getTips();
        console.log('Block IDs:', blockIds, '\n');

        // Get the metadata for the block.
        const blockMetadata = await client.getBlockMetadata(blockIds[0]);
        console.log('Block metadata: ', blockMetadata, '\n');

        // Request the block by its id.
        const blockData = await client.getBlock(blockIds[0]);
        console.log('Block data: ', blockData, '\n');
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
