// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Client, initLogger } from '@iota/sdk';
import { writeFile, readFile } from 'fs/promises';

require('dotenv').config({ path: '.env' });

// From examples directory, run with:
// yarn run-example ./client/offline_signing/01-transaction-preparation.ts

const ADDRESS_FILE_NAME = 'offline-signing-address.json';
const PREPARED_TRANSACTION_FILE_NAME =
    'offline-signing-prepared-transaction.json';

// In this example we will get inputs and prepare a transaction
async function run() {
    initLogger();
    for (const envVar of ['NODE_URL']) {
        if (!(envVar in process.env)) {
            throw new Error(`.env ${envVar} is undefined, see .env.example`);
        }
    }

    const onlineClient = new Client({
        // Insert your node URL in the .env.
        nodes: [process.env.NODE_URL as string],
        localPow: true,
    });

    const address =
        'rms1qqv5avetndkxzgr3jtrswdtz5ze6mag20s0jdqvzk4fwezve8q9vkpnqlqe';
    const amount = BigInt(1000000);
    try {
        // Recovers the address from example `0_address_generation`.
        const input_address = JSON.parse(
            await readFile(ADDRESS_FILE_NAME, 'utf8'),
        );

        // Gets enough inputs related to the address to cover the amount.
        const inputs = await onlineClient.findInputs(input_address, amount);

        // Prepares the transaction
        const preparedTransaction = await onlineClient.prepareTransaction(
            undefined,
            {
                inputs,
                output: { address, amount: amount.toString() },
            },
        );

        console.log(`Prepared transaction sending ${amount} to ${address}.`);

        await writeFile(
            PREPARED_TRANSACTION_FILE_NAME,
            JSON.stringify(preparedTransaction),
        );
    } catch (error) {
        console.error(error);
    }
}

run().then(() => process.exit());
