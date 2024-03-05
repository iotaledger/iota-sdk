// Copyright 2024 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Client, initLogger } from '@iota/sdk';
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./client/get-validators.ts [PAGE_SIZE] [CURSOR]

// This example returns the validators known by the node by querying the corresponding endpoint.
// You can provide a custom PAGE_SIZE and additionally a CURSOR from a previous request.
async function run() {
    initLogger();
    for (const envVar of ['NODE_URL']) {
        if (!(envVar in process.env)) {
            throw new Error(`.env ${envVar} is undefined, see .env.example`);
        }
    }

    const client = await Client.create({
        // Insert your node URL in the .env.
        nodes: [process.env.NODE_URL as string],
    });

    let pageSize = 1;
    let cursor = '';
    if (process.argv.length > 1) {
        pageSize = parseInt(process.argv[2]);
        if (process.argv.length > 2) {
            cursor = process.argv[3];
        }
    }

    try {
        const validators = await client.getValidators(pageSize, cursor);
        console.log(validators);
    } catch (error) {
        console.error('Error: ', error);
    }
}

void run().then(() => process.exit());
