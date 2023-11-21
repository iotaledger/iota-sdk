// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Client, initLogger } from '@iota/sdk';
import { readFile } from 'fs/promises';

require('dotenv').config({ path: '.env' });

// From examples directory, run with:
// yarn run-example ./client/offline_signing/03-send-block.ts

const SIGNED_TRANSACTION_FILE_NAME = 'offline-signing-signed-transaction.json';

// In this example we will send the signed transaction in a block.
async function run() {
    initLogger();
    for (const envVar of ['NODE_URL', 'EXPLORER_URL']) {
        if (!(envVar in process.env)) {
            throw new Error(`.env ${envVar} is undefined, see .env.example`);
        }
    }
    const onlineClient = new Client({
        // Insert your node URL in the .env.
        nodes: [process.env.NODE_URL as string],
        localPow: true,
    });

    try {
        const signedTransaction = JSON.parse(
            await readFile(SIGNED_TRANSACTION_FILE_NAME, 'utf8'),
        );

        // Send block with the signed transaction as a payload
        const blockIdAndBlock = await onlineClient.postBlockPayload(
            signedTransaction,
        );

        console.log(
            `Empty block sent: ${process.env.EXPLORER_URL}/block/${blockIdAndBlock[0]}`,
        );
    } catch (error) {
        console.error(error);
    }
}

run().then(() => process.exit());
