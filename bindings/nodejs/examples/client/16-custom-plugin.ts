// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Client, initLogger } from '@iota/sdk';
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./client/04-get-output.ts

// In this example we will get output from a known outputId
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
        const nftId =
            '0x0000000000000000000000000000000000000000000000000000000000000000';
        const route = 'outputs/nft/' + nftId;

        // Get an outputs by its id
        const output = await client.callPluginRoute(
            'api/indexer/v1/',
            'GET',
            route,
            undefined,
            undefined,
        );
        console.log('Output: ', output);
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
