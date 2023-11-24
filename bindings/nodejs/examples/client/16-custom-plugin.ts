// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Client, initLogger } from '@iota/sdk';
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./client/16-custom-plugin.ts

// In this example we will get output from a known nft by calling the node endpoint using a "custom plugin" call.
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
        localPow: true,
    });

    try {
        // Get an NFT id from ./how_tos/nfts/mint_nft.ts
        const nftId =
            '0x0000000000000000000000000000000000000000000000000000000000000000';
        const route = 'outputs/nft/' + nftId;

        // Call our "custom" indexer plugin
        const outputId = await client.callPluginRoute(
            'api/indexer/v1/',
            'GET',
            route,
            undefined,
            undefined,
        );
        console.log('Output id: ', outputId);
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
