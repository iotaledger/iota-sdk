// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import {
    Client,
    hexToUtf8,
    initLogger,
    TaggedDataPayload,
    utf8ToHex,
    Utils,
} from '@iota/sdk';
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./client/08-data-block.ts

// In this example we will send a block with a tagged data payload.
async function run() {
    initLogger();
    for (const envVar of ['NODE_URL', 'EXPLORER_URL']) {
        if (!(envVar in process.env)) {
            throw new Error(`.env ${envVar} is undefined, see .env.example`);
        }
    }

    const client = new Client({
        // Insert your node URL in the .env.
        nodes: [process.env.NODE_URL as string],
    });

    const options = {
        tag: utf8ToHex('Hello'),
        data: utf8ToHex('Tangle'),
    };
    try {
        const mnemonic = Utils.generateMnemonic();
        const secretManager = { mnemonic: mnemonic };

        // Create block with tagged payload
        const blockIdAndBlock = await client.buildAndPostBlock(
            secretManager,
            options,
        );

        console.log(
            `Block sent: ${process.env.EXPLORER_URL}/block/${blockIdAndBlock[0]}`,
        );

        const fetchedBlock = await client.getBlock(blockIdAndBlock[0]);
        console.log('Block data: ', fetchedBlock);

        if (fetchedBlock.payload instanceof TaggedDataPayload) {
            const payload = fetchedBlock.payload as TaggedDataPayload;
            console.log('Decoded data:', hexToUtf8(payload.data));
        }
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
