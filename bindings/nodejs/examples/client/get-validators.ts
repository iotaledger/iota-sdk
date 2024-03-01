// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Client, initLogger } from '@iota/sdk';
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./client/get-validators.ts

// In this example we will get the validators of the node in a paginated way.
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

    const pageSize = 1;
    const listIndex = 1;
    try {
        const slotIndex = (await client.getNodeInfo()).info.status
            .latestFinalizedSlot;
        const cursor = `${slotIndex},${listIndex}`;
        const validatorsResponse = await client.getValidators(pageSize, cursor);
        console.log(validatorsResponse);
    } catch (error) {
        console.error('Error: ', error);
    }
}

void run().then(() => process.exit());
