// Copyright 2021-2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Client, initLogger } from '@iota/sdk';
import { writeFile, readFile } from 'fs/promises';

require('dotenv').config({ path: '.env' });

// From examples directory, run with:
// yarn run-example ./client/offline_signing/02-transaction-signing.ts

const PREPARED_TRANSACTION_FILE_NAME =
    'offline-signing-prepared-transaction.json';
const SIGNED_TRANSACTION_FILE_NAME = 'offline-signing-signed-transaction.json';

// In this example we will sign the prepared transaction.
async function run() {
    initLogger();

    const offlineClient = new Client({});

    try {
        if (!process.env.MNEMONIC) {
            throw new Error('.env MNEMONIC is undefined, see .env.example');
        }

        const secretManager = {
            mnemonic: process.env.MNEMONIC,
        };

        // Read in prepared transaction from example 2_transaction_preparation
        const preparedTransaction = JSON.parse(
            await readFile(PREPARED_TRANSACTION_FILE_NAME, 'utf8'),
        );

        // Signs prepared transaction offline.
        const signedTransaction = await offlineClient.signTransaction(
            secretManager,
            preparedTransaction,
        );

        console.log('Signed transaction.');

        await writeFile(
            SIGNED_TRANSACTION_FILE_NAME,
            JSON.stringify(signedTransaction),
        );
    } catch (error) {
        console.error(error);
    }
}

run().then(() => process.exit());
