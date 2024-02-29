// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Client, ClientError, initLogger } from '@iota/sdk';

// Run with command:
// yarn run-example ./client/getting-started.ts

// In this example we will get information about the node
async function run() {
    initLogger();
    const client = await Client.create({
        nodes: ['https://api.testnet.shimmer.network'],
    });

    try {
        const info = (await client.getNodeInfo()).info;
        console.log(info);
    } catch (error) {
        if (
            error instanceof ClientError &&
            error.name === 'healthyNodePoolEmpty'
        ) {
            console.error(
                'No healthy node available, please provide a healthy one.',
            );
        } else {
            console.error(error);
        }
    }
}

void run().then(() => process.exit());
