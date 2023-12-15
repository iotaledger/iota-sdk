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
                '0x35663bfc387096d0c248f86b0877f90da5dd83b0b8163316330c3277f6d2ffb20a000000',
            ),
        );
        console.log(await client.getUtxoChangesByIndex(10));
        console.log(
            await client.getCommitment(
                '0x35663bfc387096d0c248f86b0877f90da5dd83b0b8163316330c3277f6d2ffb20a000000',
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
        // Error: node error: error decoding response body: missing field `validators` at line 1 column 29
        // console.log(await client.getValidators())
        //  Error: node error: error decoding response body: unknown variant `accepted`, expected one of `pending`, `confirmed`, `finalized`, `failed` at line 1 column 123
        // console.log(await client.getTransactionMetadata("0x6f6648fcadd2ef645935d6cc052b3804bc7ff7a5d75166e46862912fb278377259000000"))
    } catch (error) {
        console.error('Error: ', error);
    }
}

void run().then(() => process.exit());
