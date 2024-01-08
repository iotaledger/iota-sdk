// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Client, initLogger } from '@iota/sdk';

// Run with command:
// yarn run-example ./client/core-api.ts

// In this example we will send some core API requests
async function run() {
    initLogger();

    const client = await Client.create({
        nodes: ['http://localhost:8050'],
    });

    try {
        const nodeInfo = (await client.getInfo()).nodeInfo;
        console.log(nodeInfo);

        console.log(
            await client.getUtxoChanges(
                '0xa2d90c226abbefec86a12aecf4c3e04c062fa1799458bd6af5f2d1a5a89e107c09000000',
            ),
        );
        console.log(await client.getUtxoChangesByIndex(10));
        console.log(
            await client.getCommitment(
                '0xa2d90c226abbefec86a12aecf4c3e04c062fa1799458bd6af5f2d1a5a89e107c09000000',
            ),
        );
        console.log(await client.getCommitmentByIndex(10));
        console.log(
            await client.getAccountCongestion(
                '0x907c02e9302e0f0571f10f885594e56d8c54ff0708ab7a39bc1b74d396b93b12',
            ),
        );
        console.log(
            await client.getRewards(
                '0x4d8d062f5dbf65ff65be52c57c2b64a83324a3f4df143273f280e75cb4a56438050000000100',
            ),
        );
        console.log(
            await client.getValidator(
                '0x907c02e9302e0f0571f10f885594e56d8c54ff0708ab7a39bc1b74d396b93b12',
            ),
        );
        console.log(await client.getValidators());
        console.log(
            await client.getTransactionMetadata(
                '0x1afdc211c167ec58529b85a3dc848370d789bd6df16cc7d37d97021d5a37cc3917000000',
            ),
        );
    } catch (error) {
        console.error('Error: ', error);
    }
}

void run().then(() => process.exit());
