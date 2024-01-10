// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Client } from '@iota/sdk';

// Run with command:
// yarn run-example ./client/getting-started.ts

// In this example we will get information about the node
async function run() {
    const client = await Client.create({
        nodes: ['https://api.testnet.shimmer.network'],
    });

    try {
        const nodeInfo = (await client.getInfo()).nodeInfo;
        console.log(nodeInfo);
    } catch (error) {
        console.error('Error: ', error);
    }
}

void run().then(() => process.exit());
