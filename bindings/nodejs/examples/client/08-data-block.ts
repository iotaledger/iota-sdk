// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    BasicBlock,
    Client,
    hexToUtf8,
    initLogger,
    TaggedDataPayload,
    utf8ToHex,
} from '@iota/sdk';
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./client/08-data-block.ts

// In this example we will send a block with a tagged data payload.
async function run() {
    initLogger();
    if (!process.env.NODE_URL) {
        throw new Error('.env NODE_URL is undefined, see .env.example');
    }

    const client = new Client({
        // Insert your node URL in the .env.
        nodes: [process.env.NODE_URL],
    });

    try {
        // Create block with tagged payload
        const blockIdAndBlock = await client.postBlockPayload(
            new TaggedDataPayload(utf8ToHex('Hello'), utf8ToHex('Tangle')),
        );

        console.log(
            `Block sent: ${process.env.EXPLORER_URL}/block/${blockIdAndBlock[0]}`,
        );

        const fetchedBlock = await client.getBlock(blockIdAndBlock[0]);
        console.log('Block data: ', fetchedBlock);

        if (fetchedBlock instanceof BasicBlock) {
            const basic = fetchedBlock as BasicBlock;
            if (basic instanceof TaggedDataPayload) {
                const payload = basic as TaggedDataPayload;
                console.log('Decoded data:', hexToUtf8(payload.data));
            }
        }
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
