// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Client, initLogger } from '@iota/sdk';
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./client/04-get-output.ts

// In this example we will get output from a known outputId.
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
        const output = await client.getOutput(
            '0x022aefa73dff09b35b21ab5493412b0d354ad07a970a12b71e8087c6f3a7b8660000',
        );
        console.log('Output: ', output);
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
