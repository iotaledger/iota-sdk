// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

import { Wallet, initLogger } from '@iota/sdk';

// This example uses secrets in environment variables for simplicity which should not be done in production.
require('dotenv').config({ path: '.env' });

// Run with command:
// yarn run-example ./how_tos/accounts_and_addresses/check-balance.ts

// This example syncs the account and prints the balance.
async function run() {
    initLogger();
    for (const envVar of ['WALLET_DB_PATH']) {
        if (!(envVar in process.env)) {
            throw new Error(`.env ${envVar} is undefined, see .env.example`);
        }
    }
    try {
        const wallet = new Wallet({
            storagePath: process.env.WALLET_DB_PATH,
        });

        const account = await wallet.getAccount('Alice');

        // Sync new outputs from the node.
        // eslint-disable-next-line @typescript-eslint/no-unused-vars
        const _syncBalance = await account.sync();

        // After syncing the balance can also be computed with the local data
        const balance = await account.getBalance();
        console.log('Balance', balance);
    } catch (error) {
        console.error('Error: ', error);
    }
}

run().then(() => process.exit());
